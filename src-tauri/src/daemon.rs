use crate::{portal, state, wifi};
use std::time::{Duration, Instant};

struct DaemonLock {
    #[cfg(target_os = "windows")]
    handle: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(not(target_os = "windows"))]
    pid_path: Option<std::path::PathBuf>,
}

impl Drop for DaemonLock {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        unsafe {
            if self.handle != 0 {
                windows_sys::Win32::Foundation::CloseHandle(self.handle);
            }
        }
        #[cfg(not(target_os = "windows"))]
        if let Some(ref p) = self.pid_path {
            let _ = std::fs::remove_file(p);
        }
    }
}

fn acquire_daemon_lock() -> Option<DaemonLock> {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
        use windows_sys::Win32::System::Threading::CreateMutexW;

        let name: Vec<u16> = "Local\\CiupWifiDaemonMutex\0".encode_utf16().collect();
        let handle = CreateMutexW(std::ptr::null(), 0, name.as_ptr());
        if handle == 0 {
            return None;
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            CloseHandle(handle);
            return None;
        }
        return Some(DaemonLock { handle });
    }

    #[cfg(not(target_os = "windows"))]
    {
        let dir = state::get_app_dir(None);
        let _ = std::fs::create_dir_all(&dir);
        let pid_path = dir.join("daemon.pid");

        if let Ok(existing) = std::fs::read_to_string(&pid_path) {
            let pid_str = existing.trim();
            if !pid_str.is_empty() {
                if let Ok(status) = std::process::Command::new("kill")
                    .args(["-0", pid_str])
                    .status()
                {
                    if status.success() {
                        return None;
                    }
                }
            }
        }

        let _ = std::fs::write(&pid_path, std::process::id().to_string());
        Some(DaemonLock {
            pid_path: Some(pid_path),
        })
    }
}

pub fn run_daemon() {
    let _lock = match acquire_daemon_lock() {
        Some(l) => l,
        None => return,
    };

    tauri::async_runtime::block_on(async move {
        let poll_interval = if cfg!(target_os = "windows") {
            Duration::from_secs(3)
        } else {
            Duration::from_secs(5)
        };

        let mut was_on_wificity = false;
        let mut last_probe_at: Option<Instant> = None;
        let mut consecutive_failures: u32 = 0;

        loop {
            let mut app_state = state::load_state(None);
            if !app_state.auto_enabled {
                break;
            }

            let wifi_info = wifi::get_current_wifi_info();
            let on_wificity = match wifi_info.ssid {
                Some(ref ssid) => wifi::is_wificity_ssid(ssid),
                None if wifi_info.is_wifi && wifi_info.location_blocked => wifi_info
                    .gateway_ip
                    .as_deref()
                    .map(wifi::is_ciup_gateway)
                    .unwrap_or(false),
                None => false,
            };

            if !was_on_wificity && on_wificity {
                // Freshly joined WifiCity: clear manual pause / auth error and probe right away
                if app_state.user_paused || app_state.auth_failed || app_state.last_error.is_some()
                {
                    app_state.user_paused = false;
                    app_state.auth_failed = false;
                    app_state.last_error = None;
                    let _ = state::save_state(None, &app_state);
                }
                consecutive_failures = 0;
                last_probe_at = None;
            } else if was_on_wificity && !on_wificity {
                // Left WifiCity: reset pause/auth flags so next join auto-connects
                if app_state.user_paused || app_state.auth_failed || app_state.last_error.is_some()
                {
                    app_state.user_paused = false;
                    app_state.auth_failed = false;
                    app_state.last_error = None;
                    let _ = state::save_state(None, &app_state);
                }
                consecutive_failures = 0;
                last_probe_at = None;
            }

            was_on_wificity = on_wificity;

            if on_wificity && !app_state.user_paused && !app_state.auth_failed {
                let due = match last_probe_at {
                    None => true,
                    Some(prev) => {
                        let wait_secs = if consecutive_failures == 0 {
                            30
                        } else {
                            let backoff = [5u64, 10, 20, 30];
                            let idx = ((consecutive_failures - 1) as usize).min(backoff.len() - 1);
                            backoff[idx]
                        };
                        prev.elapsed() >= Duration::from_secs(wait_secs)
                    }
                };

                if due {
                    last_probe_at = Some(Instant::now());
                    if let Some(creds) = state::load_credentials_direct(None) {
                        let gw = wifi_info.gateway_ip.as_deref();
                        match portal::probe_connectivity(gw).await {
                            portal::ProbeResult::Online => {
                                consecutive_failures = 0;
                            }
                            portal::ProbeResult::Captive { .. }
                            | portal::ProbeResult::Offline => {
                                match portal::login_to_portal(&creds.username, &creds.password, gw)
                                    .await
                                {
                                    portal::LoginOutcome::Success {
                                        portal_url,
                                        logout_url,
                                    } => {
                                        consecutive_failures = 0;
                                        let mut fresh = state::load_state(None);
                                        if portal_url.is_some() {
                                            fresh.last_portal_url = portal_url;
                                        }
                                        if logout_url.is_some() {
                                            fresh.last_logout_url = logout_url;
                                        }
                                        fresh.auth_failed = false;
                                        fresh.last_error = None;
                                        let _ = state::save_state(None, &fresh);
                                    }
                                    portal::LoginOutcome::AuthFailed(msg) => {
                                        let mut fresh = state::load_state(None);
                                        fresh.auth_failed = true;
                                        fresh.last_error = Some(msg);
                                        let _ = state::save_state(None, &fresh);
                                    }
                                    portal::LoginOutcome::Error(msg) => {
                                        consecutive_failures = consecutive_failures.saturating_add(1);
                                        let mut fresh = state::load_state(None);
                                        fresh.last_error = Some(msg);
                                        let _ = state::save_state(None, &fresh);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(poll_interval).await;
        }
    });
}

pub fn setup_background_task_internal(enable: bool) -> Result<(), String> {
    let mut app_state = state::load_state(None);
    app_state.auto_enabled = enable;
    state::save_state(None, &app_state)?;

    #[cfg(target_os = "android")]
    {
        return Ok(());
    }

    #[cfg(not(target_os = "android"))]
    {
        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let exe_path = current_exe.to_string_lossy().to_string();

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            const DETACHED_PROCESS: u32 = 0x00000008;

            // Remove legacy periodic tasks so we don't stack old --silent runs
            for legacy in ["CiupWifiAuto", "CiupWifiLogon"] {
                let _ = std::process::Command::new("schtasks")
                    .args(["/delete", "/tn", legacy, "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }

            if enable {
                let _ = std::process::Command::new("schtasks")
                    .args([
                        "/create",
                        "/tn",
                        "CiupWifiDaemon",
                        "/tr",
                        &format!("\"{}\" --daemon", exe_path),
                        "/sc",
                        "onlogon",
                        "/f",
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();

                let _ = std::process::Command::new(&current_exe)
                    .arg("--daemon")
                    .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
                    .spawn();
            } else {
                let _ = std::process::Command::new("schtasks")
                    .args(["/delete", "/tn", "CiupWifiDaemon", "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let plist_path = std::path::PathBuf::from(home)
                    .join("Library/LaunchAgents/org.ciup.wifi.plist");
                if enable {
                    if let Some(parent) = plist_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let plist_content = format!(
                        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>org.ciup.wifi</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>--daemon</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>ThrottleInterval</key>
    <integer>10</integer>
</dict>
</plist>"#,
                        exe_path
                    );
                    let _ = std::fs::write(&plist_path, plist_content);
                    let _ = std::process::Command::new("launchctl")
                        .args(["unload", plist_path.to_str().unwrap_or("")])
                        .output();
                    let _ = std::process::Command::new("launchctl")
                        .args(["load", "-w", plist_path.to_str().unwrap_or("")])
                        .output();
                } else {
                    let _ = std::process::Command::new("launchctl")
                        .args(["unload", plist_path.to_str().unwrap_or("")])
                        .output();
                    let _ = std::fs::remove_file(&plist_path);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let autostart_dir = std::path::PathBuf::from(home).join(".config/autostart");
                let desktop_path = autostart_dir.join("ciupwifi-daemon.desktop");
                if enable {
                    let _ = std::fs::create_dir_all(&autostart_dir);
                    let content = format!(
                        "[Desktop Entry]\nType=Application\nName=CiupWifi Daemon\nExec=\"{}\" --daemon\nX-GNOME-Autostart-enabled=true\n",
                        exe_path
                    );
                    let _ = std::fs::write(&desktop_path, content);
                    let _ = std::process::Command::new(&current_exe)
                        .arg("--daemon")
                        .spawn();
                } else {
                    let _ = std::fs::remove_file(&desktop_path);
                }
            }
        }

        Ok(())
    }
}
