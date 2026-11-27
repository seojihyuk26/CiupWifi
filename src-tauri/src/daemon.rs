use crate::{portal, state, wifi};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Notify;

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

        let mutex_name = format!("Local\\CiupWifiDaemonMutex_{}\0", env!("CARGO_PKG_VERSION"));
        let name: Vec<u16> = mutex_name.encode_utf16().collect();
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

#[cfg(target_os = "windows")]
struct WlanWatcher {
    handle: windows_sys::Win32::Foundation::HANDLE,
    ctx_ptr: *const Notify,
}

#[cfg(target_os = "windows")]
unsafe impl Send for WlanWatcher {}

#[cfg(target_os = "windows")]
impl Drop for WlanWatcher {
    fn drop(&mut self) {
        use windows_sys::Win32::NetworkManagement::WiFi::{
            WlanCloseHandle, WlanRegisterNotification,
        };
        unsafe {
            WlanRegisterNotification(
                self.handle,
                0,
                0,
                None,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
            );
            WlanCloseHandle(self.handle, std::ptr::null());
            drop(Arc::from_raw(self.ctx_ptr));
        }
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn wlan_notify_callback(
    data: *mut windows_sys::Win32::NetworkManagement::WiFi::L2_NOTIFICATION_DATA,
    ctx: *mut std::ffi::c_void,
) {
    use windows_sys::Win32::NetworkManagement::WiFi::{
        wlan_notification_acm_connection_attempt_fail, wlan_notification_acm_connection_complete,
        wlan_notification_acm_disconnected,
    };

    if ctx.is_null() || data.is_null() {
        return;
    }
    let code = (*data).NotificationCode as i32;
    if code == wlan_notification_acm_connection_complete
        || code == wlan_notification_acm_disconnected
        || code == wlan_notification_acm_connection_attempt_fail
    {
        let notify = &*(ctx as *const Notify);
        notify.notify_one();
    }
}

#[cfg(target_os = "windows")]
fn start_wlan_watcher(notify: &Arc<Notify>) -> Option<WlanWatcher> {
    use windows_sys::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
    use windows_sys::Win32::NetworkManagement::WiFi::{
        WlanCloseHandle, WlanOpenHandle, WlanRegisterNotification, WLAN_NOTIFICATION_SOURCE_ACM,
    };

    unsafe {
        let mut negotiated: u32 = 0;
        let mut handle: HANDLE = 0 as HANDLE;
        if WlanOpenHandle(2, std::ptr::null(), &mut negotiated, &mut handle) != ERROR_SUCCESS {
            return None;
        }
        let raw_ctx = Arc::into_raw(Arc::clone(notify));
        let res = WlanRegisterNotification(
            handle,
            WLAN_NOTIFICATION_SOURCE_ACM,
            1,
            Some(wlan_notify_callback),
            raw_ctx as *const std::ffi::c_void,
            std::ptr::null(),
            std::ptr::null_mut(),
        );
        if res != ERROR_SUCCESS {
            drop(Arc::from_raw(raw_ctx));
            WlanCloseHandle(handle, std::ptr::null());
            return None;
        }
        Some(WlanWatcher {
            handle,
            ctx_ptr: raw_ctx,
        })
    }
}

#[cfg(target_os = "macos")]
mod macos_watcher {
    use std::ffi::c_void;
    use std::os::raw::c_char;
    use std::sync::Arc;
    use tokio::sync::Notify;

    type CFIndex = isize;
    type CFTypeRef = *const c_void;
    type CFAllocatorRef = *const c_void;
    type CFStringRef = *const c_void;
    type CFArrayRef = *const c_void;
    type CFRunLoopRef = *const c_void;
    type CFRunLoopSourceRef = *const c_void;
    type SCDynamicStoreRef = *const c_void;
    type Boolean = u8;

    #[repr(C)]
    struct SCDynamicStoreContext {
        version: CFIndex,
        info: *mut c_void,
        retain: Option<extern "C" fn(*const c_void) -> *const c_void>,
        release: Option<extern "C" fn(*const c_void)>,
        copy_description: Option<extern "C" fn(*const c_void) -> CFStringRef>,
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFAllocatorDefault: CFAllocatorRef;
        static kCFTypeArrayCallBacks: c_void;
        static kCFRunLoopDefaultMode: CFStringRef;
        fn CFStringCreateWithCString(
            alloc: CFAllocatorRef,
            c_str: *const c_char,
            encoding: u32,
        ) -> CFStringRef;
        fn CFArrayCreate(
            alloc: CFAllocatorRef,
            values: *const *const c_void,
            num_values: CFIndex,
            callbacks: *const c_void,
        ) -> CFArrayRef;
        fn CFRelease(cf: CFTypeRef);
        fn CFRunLoopGetCurrent() -> CFRunLoopRef;
        fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
        fn CFRunLoopRun();
    }

    #[link(name = "SystemConfiguration", kind = "framework")]
    extern "C" {
        fn SCDynamicStoreCreate(
            allocator: CFAllocatorRef,
            name: CFStringRef,
            callout: Option<extern "C" fn(SCDynamicStoreRef, CFArrayRef, *mut c_void)>,
            context: *mut SCDynamicStoreContext,
        ) -> SCDynamicStoreRef;
        fn SCDynamicStoreSetNotificationKeys(
            store: SCDynamicStoreRef,
            keys: CFArrayRef,
            patterns: CFArrayRef,
        ) -> Boolean;
        fn SCDynamicStoreCreateRunLoopSource(
            allocator: CFAllocatorRef,
            store: SCDynamicStoreRef,
            order: CFIndex,
        ) -> CFRunLoopSourceRef;
    }

    const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

    extern "C" fn store_callback(
        _store: SCDynamicStoreRef,
        _changed_keys: CFArrayRef,
        info: *mut c_void,
    ) {
        if !info.is_null() {
            unsafe {
                let notify = &*(info as *const Notify);
                notify.notify_one();
            }
        }
    }

    pub fn spawn_network_watcher(notify: Arc<Notify>) {
        std::thread::spawn(move || unsafe {
            let name = CFStringCreateWithCString(
                kCFAllocatorDefault,
                b"org.ciup.wifi.watcher\0".as_ptr() as *const c_char,
                K_CF_STRING_ENCODING_UTF8,
            );
            let raw_notify = Arc::into_raw(notify) as *mut c_void;
            let mut ctx = SCDynamicStoreContext {
                version: 0,
                info: raw_notify,
                retain: None,
                release: None,
                copy_description: None,
            };
            let store = SCDynamicStoreCreate(
                kCFAllocatorDefault,
                name,
                Some(store_callback),
                &mut ctx,
            );
            if !name.is_null() {
                CFRelease(name);
            }
            if store.is_null() {
                drop(Arc::from_raw(raw_notify as *const Notify));
                return;
            }

            let pat1 = CFStringCreateWithCString(
                kCFAllocatorDefault,
                b"State:/Network/Interface/.*/IPv4\0".as_ptr() as *const c_char,
                K_CF_STRING_ENCODING_UTF8,
            );
            let pat2 = CFStringCreateWithCString(
                kCFAllocatorDefault,
                b"State:/Network/Global/IPv4\0".as_ptr() as *const c_char,
                K_CF_STRING_ENCODING_UTF8,
            );
            let vals = [pat1, pat2];
            let patterns = CFArrayCreate(
                kCFAllocatorDefault,
                vals.as_ptr(),
                2,
                &kCFTypeArrayCallBacks as *const c_void,
            );
            let ok = SCDynamicStoreSetNotificationKeys(store, std::ptr::null(), patterns);
            CFRelease(pat1);
            CFRelease(pat2);
            CFRelease(patterns);

            if ok == 0 {
                CFRelease(store);
                drop(Arc::from_raw(raw_notify as *const Notify));
                return;
            }

            let rl_src = SCDynamicStoreCreateRunLoopSource(kCFAllocatorDefault, store, 0);
            if rl_src.is_null() {
                CFRelease(store);
                drop(Arc::from_raw(raw_notify as *const Notify));
                return;
            }

            let rl = CFRunLoopGetCurrent();
            CFRunLoopAddSource(rl, rl_src, kCFRunLoopDefaultMode);
            CFRelease(rl_src);
            CFRunLoopRun();
        });
    }
}

async fn wait_for_dhcp_settle() {
    for _ in 0..10 {
        let info = wifi::get_current_wifi_info();
        if !info.is_wifi || info.gateway_ip.is_some() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

pub fn run_daemon() {
    let _lock = match acquire_daemon_lock() {
        Some(l) => l,
        None => return,
    };

    state::append_daemon_log(&format!("Daemon started (v{})", env!("CARGO_PKG_VERSION")));

    tauri::async_runtime::block_on(async move {
        let notify = Arc::new(Notify::new());
        #[cfg(target_os = "windows")]
        let _watcher = start_wlan_watcher(&notify);
        #[cfg(target_os = "macos")]
        macos_watcher::spawn_network_watcher(Arc::clone(&notify));

        let mut was_on_wificity = false;
        let mut last_probe_at: Option<Instant> = None;
        let mut consecutive_failures: u32 = 0;
        let mut unknown_ssid_probed = false;

        loop {
            let mut app_state = state::load_state(None);
            if !app_state.auto_enabled {
                state::append_daemon_log("Daemon stopping (auto_enabled=false)");
                break;
            }
            if let Some(ref expected_ver) = app_state.daemon_version {
                if expected_ver != env!("CARGO_PKG_VERSION") {
                    state::append_daemon_log(&format!(
                        "Daemon exiting for upgrade ({} -> {})",
                        env!("CARGO_PKG_VERSION"),
                        expected_ver
                    ));
                    break;
                }
            }

            let wifi_info = wifi::get_current_wifi_info();
            let gw = wifi_info.gateway_ip.as_deref();
            let mut cached_probe: Option<portal::ProbeResult> = None;

            let on_wificity = match wifi::is_known_wificity_without_probe(&wifi_info, &app_state) {
                Some(b) => {
                    unknown_ssid_probed = false;
                    b
                }
                None => {
                    if !unknown_ssid_probed {
                        let probe = portal::probe_connectivity(gw).await;
                        match &probe {
                            portal::ProbeResult::Online => {
                                unknown_ssid_probed = true;
                                false
                            }
                            portal::ProbeResult::Captive {
                                portal_url,
                                initial_html,
                            } => {
                                if portal::is_ciup_portal(
                                    gw,
                                    Some(portal_url),
                                    initial_html.as_deref(),
                                    &app_state.known_ciup_gateways,
                                ) {
                                    app_state.record_ciup_gateway(gw);
                                    let _ = state::save_state(None, &app_state);
                                    cached_probe = Some(probe);
                                    true
                                } else {
                                    unknown_ssid_probed = true;
                                    false
                                }
                            }
                            portal::ProbeResult::Offline => {
                                // Do not lock unknown_ssid_probed on Offline (DHCP/portal may still be settling)
                                false
                            }
                        }
                    } else {
                        false
                    }
                }
            };

            if !was_on_wificity && on_wificity {
                state::append_daemon_log("Joined WifiCity; arming auto-login");
                app_state.was_on_wificity = true;
                app_state.user_paused = false;
                app_state.auth_failed = false;
                app_state.last_error = None;
                let _ = state::save_state(None, &app_state);
                consecutive_failures = 0;
                last_probe_at = None;
            } else if was_on_wificity && !on_wificity {
                state::append_daemon_log("Left WifiCity; resetting pause state");
                app_state.was_on_wificity = false;
                app_state.user_paused = false;
                app_state.auth_failed = false;
                app_state.last_error = None;
                let _ = state::save_state(None, &app_state);
                consecutive_failures = 0;
                last_probe_at = None;
            }

            was_on_wificity = on_wificity;

            let mut next_wait = if cfg!(any(target_os = "windows", target_os = "macos")) {
                Duration::from_secs(60)
            } else {
                Duration::from_secs(15)
            };

            if on_wificity && !app_state.user_paused && !app_state.auth_failed {
                let cadence_secs = if consecutive_failures == 0 {
                    30u64
                } else {
                    let backoff = [5u64, 15, 30, 60];
                    let idx = ((consecutive_failures - 1) as usize).min(backoff.len() - 1);
                    backoff[idx]
                };

                let due = match last_probe_at {
                    None => true,
                    Some(prev) => prev.elapsed() >= Duration::from_secs(cadence_secs),
                };

                if due {
                    last_probe_at = Some(Instant::now());
                    if let Some(creds) = state::load_credentials_direct(None) {
                        let probe_res = match cached_probe.take() {
                            Some(p) => p,
                            None => portal::probe_connectivity(gw).await,
                        };
                        match probe_res {
                            portal::ProbeResult::Online => {
                                consecutive_failures = 0;
                            }
                            other @ (portal::ProbeResult::Captive { .. }
                            | portal::ProbeResult::Offline) => {
                                match portal::login_with_probe(
                                    &creds.username,
                                    &creds.password,
                                    gw,
                                    Some(other),
                                )
                                .await
                                {
                                    portal::LoginOutcome::Success {
                                        portal_url,
                                        logout_url,
                                    } => {
                                        state::append_daemon_log("Auto-login succeeded");
                                        consecutive_failures = 0;
                                        let mut fresh = state::load_state(None);
                                        fresh.record_ciup_gateway(gw);
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
                                        state::append_daemon_log(&format!("Auth failed: {}", msg));
                                        let mut fresh = state::load_state(None);
                                        fresh.auth_failed = true;
                                        fresh.last_error = Some(msg);
                                        let _ = state::save_state(None, &fresh);
                                    }
                                    portal::LoginOutcome::Error(msg) => {
                                        state::append_daemon_log(&format!("Login retry error: {}", msg));
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

                let wait_secs = if consecutive_failures == 0 {
                    30u64
                } else {
                    let backoff = [5u64, 15, 30, 60];
                    let idx = ((consecutive_failures - 1) as usize).min(backoff.len() - 1);
                    backoff[idx]
                };
                next_wait = Duration::from_secs(wait_secs);
            }

            tokio::select! {
                _ = notify.notified() => {
                    unknown_ssid_probed = false;
                    last_probe_at = None;
                    wait_for_dhcp_settle().await;
                }
                _ = tokio::time::sleep(next_wait) => {}
            }
        }
    });
}

#[cfg(target_os = "macos")]
fn resolve_macos_exe(current_exe: &std::path::Path) -> String {
    let raw = current_exe.to_string_lossy().to_string();
    if raw.contains("AppTranslocation") || raw.contains("/Volumes/") {
        let installed = std::path::Path::new("/Applications/CiupWifi.app/Contents/MacOS/ciupwifi");
        if installed.exists() {
            return installed.to_string_lossy().to_string();
        }
    }
    raw
}

pub fn setup_background_task_internal(enable: bool) -> Result<(), String> {
    let mut app_state = state::load_state(None);
    app_state.auto_enabled = enable;
    app_state.daemon_version = Some(env!("CARGO_PKG_VERSION").to_string());
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

            for legacy in ["CiupWifiAuto", "CiupWifiLogon", "CiupWifiDaemon"] {
                let _ = std::process::Command::new("schtasks")
                    .args(["/delete", "/tn", legacy, "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }

            if enable {
                let _ = std::process::Command::new("reg")
                    .args([
                        "add",
                        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                        "/v",
                        "CiupWifi",
                        "/t",
                        "REG_SZ",
                        "/d",
                        &format!("\"{}\" --daemon", exe_path),
                        "/f",
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();

                // Do NOT combine CREATE_NO_WINDOW with DETACHED_PROCESS (Windows ignores CREATE_NO_WINDOW)
                let _ = std::process::Command::new(&current_exe)
                    .arg("--daemon")
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn();
            } else {
                let _ = std::process::Command::new("reg")
                    .args([
                        "delete",
                        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
                        "/v",
                        "CiupWifi",
                        "/f",
                    ])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }
        }

        #[cfg(target_os = "macos")]
        {
            let stable_exe = resolve_macos_exe(&current_exe);
            if let Ok(home) = std::env::var("HOME") {
                let plist_path = std::path::PathBuf::from(home)
                    .join("Library/LaunchAgents/org.ciup.wifi.plist");
                let plist_str = plist_path.to_str().unwrap_or("");
                let uid_out = std::process::Command::new("id").arg("-u").output();
                let domain_target = uid_out
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| format!("gui/{}", s.trim()))
                    .unwrap_or_else(|| "gui/501".to_string());
                let service_target = format!("{}/org.ciup.wifi", domain_target);

                if enable {
                    if let Some(parent) = plist_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    // Event-driven daemon using native SCDynamicStore (0% CPU when idle)
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
    <integer>5</integer>
</dict>
</plist>"#,
                        stable_exe
                    );
                    let _ = std::fs::write(&plist_path, plist_content);
                    let _ = std::process::Command::new("launchctl")
                        .args(["bootout", &domain_target, plist_str])
                        .output();
                    let boot = std::process::Command::new("launchctl")
                        .args(["bootstrap", &domain_target, plist_str])
                        .output();
                    if boot.map(|o| o.status.success()).unwrap_or(false) {
                        let _ = std::process::Command::new("launchctl")
                            .args(["kickstart", "-k", &service_target])
                            .output();
                    } else {
                        let _ = std::process::Command::new("launchctl")
                            .args(["unload", plist_str])
                            .output();
                        let _ = std::process::Command::new("launchctl")
                            .args(["load", "-w", plist_str])
                            .output();
                    }
                } else {
                    let _ = std::process::Command::new("launchctl")
                        .args(["bootout", &domain_target, plist_str])
                        .output();
                    let _ = std::process::Command::new("launchctl")
                        .args(["unload", plist_str])
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
