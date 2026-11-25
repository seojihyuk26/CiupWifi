pub mod daemon;
pub mod portal;
pub mod state;
pub mod wifi;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::commands::get_network_status,
            crate::commands::login_to_portal,
            crate::commands::disconnect_wificity,
            crate::commands::check_connectivity,
            crate::commands::save_credentials,
            crate::commands::load_credentials,
            crate::commands::setup_background_task,
            crate::commands::check_for_updates,
            crate::commands::download_and_install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn run_daemon() {
    daemon::run_daemon();
}

pub fn run_headless() {
    tauri::async_runtime::block_on(async {
        let app_state = state::load_state(None);
        if app_state.user_paused || !app_state.auto_enabled {
            return;
        }

        let wifi_info = wifi::get_current_wifi_info();
        let on_wificity = match wifi_info.ssid {
            Some(ref s) => wifi::is_wificity_ssid(s),
            None if wifi_info.is_wifi && wifi_info.location_blocked => wifi_info
                .gateway_ip
                .as_deref()
                .map(wifi::is_ciup_gateway)
                .unwrap_or(false),
            None => false,
        };

        if !on_wificity {
            return;
        }

        let creds = match state::load_credentials_direct(None) {
            Some(c) => c,
            None => {
                eprintln!("[CiupWifi] No saved credentials found. Please open CiupWifi once to save your login.");
                return;
            }
        };

        let gw = wifi_info.gateway_ip.as_deref();
        if matches!(portal::probe_connectivity(gw).await, portal::ProbeResult::Online) {
            println!("[CiupWifi] WifiCity internet is already active.");
            return;
        }

        println!("[CiupWifi] Captive portal detected on WifiCity. Logging in as {}...", creds.username);
        match portal::login_to_portal(&creds.username, &creds.password, gw).await {
            portal::LoginOutcome::Success { portal_url, logout_url } => {
                let mut st = state::load_state(None);
                if portal_url.is_some() {
                    st.last_portal_url = portal_url;
                }
                if logout_url.is_some() {
                    st.last_logout_url = logout_url;
                }
                st.auth_failed = false;
                st.last_error = None;
                let _ = state::save_state(None, &st);
                println!("[CiupWifi] Login successful!");
            }
            portal::LoginOutcome::AuthFailed(msg) => {
                let mut st = state::load_state(None);
                st.auth_failed = true;
                st.last_error = Some(msg.clone());
                let _ = state::save_state(None, &st);
                eprintln!("[CiupWifi] {}", msg);
            }
            portal::LoginOutcome::Error(msg) => {
                eprintln!("[CiupWifi] Connection error: {}", msg);
            }
        }
    });
}

pub mod commands {
    use crate::{daemon, portal, state, wifi};
    use reqwest::Client;

    fn evaluate_wificity(wifi_info: &wifi::WifiInfo) -> (bool, bool) {
        if let Some(ref ssid) = wifi_info.ssid {
            (wifi::is_wificity_ssid(ssid), false)
        } else if wifi_info.is_wifi && wifi_info.location_blocked {
            let ciup_gw = wifi_info
                .gateway_ip
                .as_deref()
                .map(wifi::is_ciup_gateway)
                .unwrap_or(false);
            (ciup_gw, !ciup_gw)
        } else {
            (false, false)
        }
    }

    #[tauri::command]
    pub async fn get_network_status(app: tauri::AppHandle) -> Result<state::NetworkStatus, String> {
        let app_state = state::load_state(Some(&app));
        let wifi_info = wifi::get_current_wifi_info();
        let (is_wificity, needs_location_perm) = evaluate_wificity(&wifi_info);

        if needs_location_perm {
            return Ok(state::NetworkStatus {
                state: "permission_needed".to_string(),
                ssid: None,
                is_wifi: true,
                is_cellular: false,
                is_wificity: false,
                user_paused: app_state.user_paused,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: app_state.last_portal_url,
                detail: Some(
                    "Enable Location access in OS settings so CiupWifi can verify the Wi-Fi network name (WifiCity)."
                        .to_string(),
                ),
            });
        }

        if !wifi_info.is_wifi {
            return Ok(state::NetworkStatus {
                state: "disconnected".to_string(),
                ssid: None,
                is_wifi: false,
                is_cellular: false,
                is_wificity: false,
                user_paused: app_state.user_paused,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: None,
                detail: Some("Wi-Fi is not connected.".to_string()),
            });
        }

        if !is_wificity {
            return Ok(state::NetworkStatus {
                state: "not_on_wificity".to_string(),
                ssid: wifi_info.ssid,
                is_wifi: true,
                is_cellular: false,
                is_wificity: false,
                user_paused: app_state.user_paused,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: None,
                detail: Some("Not connected to WifiCity.".to_string()),
            });
        }

        if app_state.user_paused {
            return Ok(state::NetworkStatus {
                state: "paused".to_string(),
                ssid: wifi_info.ssid.or_else(|| Some("WifiCity".to_string())),
                is_wifi: true,
                is_cellular: false,
                is_wificity: true,
                user_paused: true,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: app_state.last_portal_url,
                detail: Some("Disconnected by user. Tap Connect or rejoin WifiCity to resume.".to_string()),
            });
        }

        let gw = wifi_info.gateway_ip.as_deref();
        match portal::probe_connectivity(gw).await {
            portal::ProbeResult::Online => Ok(state::NetworkStatus {
                state: "wifi_city_online".to_string(),
                ssid: wifi_info.ssid.or_else(|| Some("WifiCity".to_string())),
                is_wifi: true,
                is_cellular: false,
                is_wificity: true,
                user_paused: false,
                auto_enabled: app_state.auto_enabled,
                auth_failed: false,
                portal_url: app_state.last_portal_url,
                detail: None,
            }),
            portal::ProbeResult::Captive { portal_url, .. } => Ok(state::NetworkStatus {
                state: "wifi_city_captive".to_string(),
                ssid: wifi_info.ssid.or_else(|| Some("WifiCity".to_string())),
                is_wifi: true,
                is_cellular: false,
                is_wificity: true,
                user_paused: false,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: Some(portal_url),
                detail: app_state.last_error,
            }),
            portal::ProbeResult::Offline => Ok(state::NetworkStatus {
                state: "wifi_city_captive".to_string(),
                ssid: wifi_info.ssid.or_else(|| Some("WifiCity".to_string())),
                is_wifi: true,
                is_cellular: false,
                is_wificity: true,
                user_paused: false,
                auto_enabled: app_state.auto_enabled,
                auth_failed: app_state.auth_failed,
                portal_url: app_state.last_portal_url,
                detail: Some("Connected to WifiCity, waiting for internet or portal...".to_string()),
            }),
        }
    }

    #[tauri::command]
    pub async fn login_to_portal(
        app: tauri::AppHandle,
        username: String,
        password: String,
    ) -> Result<serde_json::Value, String> {
        let wifi_info = wifi::get_current_wifi_info();
        let (is_wificity, needs_location_perm) = evaluate_wificity(&wifi_info);

        if needs_location_perm {
            return Ok(serde_json::json!({
                "ok": false,
                "reason": "permission_needed",
                "auth_failed": false,
                "message": "Enable Location access in OS settings so CiupWifi can verify WifiCity."
            }));
        }

        if !wifi_info.is_wifi || !is_wificity {
            let msg = match wifi_info.ssid {
                Some(ref s) => format!("Currently connected to '{}' (not WifiCity). Switch to WifiCity first.", s),
                None => "Not connected to the WifiCity Wi-Fi network.".to_string(),
            };
            return Ok(serde_json::json!({
                "ok": false,
                "reason": "not_on_wificity",
                "auth_failed": false,
                "message": msg
            }));
        }

        let mut app_state = state::load_state(Some(&app));
        app_state.user_paused = false;
        app_state.auth_failed = false;
        app_state.last_error = None;
        let _ = state::save_state(Some(&app), &app_state);

        let gw = wifi_info.gateway_ip.as_deref();
        match portal::login_to_portal(&username, &password, gw).await {
            portal::LoginOutcome::Success {
                portal_url,
                logout_url,
            } => {
                if let Some(ref p) = portal_url {
                    app_state.last_portal_url = Some(p.clone());
                }
                if let Some(ref l) = logout_url {
                    app_state.last_logout_url = Some(l.clone());
                }
                app_state.auth_failed = false;
                app_state.last_error = None;
                let _ = state::save_state(Some(&app), &app_state);

                Ok(serde_json::json!({
                    "ok": true,
                    "auth_failed": false,
                    "portal_url": portal_url,
                    "message": "Connected to WifiCity."
                }))
            }
            portal::LoginOutcome::AuthFailed(msg) => {
                app_state.auth_failed = true;
                app_state.last_error = Some(msg.clone());
                let _ = state::save_state(Some(&app), &app_state);

                Ok(serde_json::json!({
                    "ok": false,
                    "reason": "auth_failed",
                    "auth_failed": true,
                    "message": msg
                }))
            }
            portal::LoginOutcome::Error(msg) => {
                app_state.last_error = Some(msg.clone());
                let _ = state::save_state(Some(&app), &app_state);

                Ok(serde_json::json!({
                    "ok": false,
                    "reason": "error",
                    "auth_failed": false,
                    "message": msg
                }))
            }
        }
    }

    #[tauri::command]
    pub async fn disconnect_wificity(app: tauri::AppHandle) -> Result<(), String> {
        let mut app_state = state::load_state(Some(&app));
        app_state.user_paused = true;
        let logout_url = app_state.last_logout_url.clone();
        state::save_state(Some(&app), &app_state)?;

        if let Some(url) = logout_url {
            portal::logout_from_portal(&url).await;
        }
        wifi::disconnect_wifi()?;
        Ok(())
    }

    #[tauri::command]
    pub async fn check_connectivity(app: tauri::AppHandle) -> Result<String, String> {
        let status = get_network_status(app).await?;
        if status.state == "wifi_city_online" {
            Ok("connected".to_string())
        } else {
            Ok(status.state)
        }
    }

    #[tauri::command]
    pub fn save_credentials(
        app: tauri::AppHandle,
        username: String,
        password: String,
    ) -> Result<(), String> {
        state::save_credentials_direct(Some(&app), &username, &password)?;
        let mut app_state = state::load_state(Some(&app));
        app_state.auth_failed = false;
        app_state.last_error = None;
        let _ = state::save_state(Some(&app), &app_state);
        Ok(())
    }

    #[tauri::command]
    pub fn load_credentials(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
        match state::load_credentials_direct(Some(&app)) {
            Some(c) => Ok(serde_json::json!({
                "username": c.username,
                "password": c.password,
            })),
            None => Ok(serde_json::json!({
                "username": "",
                "password": "",
            })),
        }
    }

    #[tauri::command]
    pub fn setup_background_task(enable: bool) -> Result<(), String> {
        daemon::setup_background_task_internal(enable)
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    pub struct UpdateInfo {
        pub current_version: String,
        pub latest_version: String,
        pub download_url: String,
        pub asset_name: String,
        pub release_notes: String,
    }

    fn parse_version_parts(v: &str) -> Vec<u32> {
        v.trim_start_matches('v')
            .trim()
            .split('.')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    }

    fn is_newer_version(latest: &str, current: &str) -> bool {
        let l_parts = parse_version_parts(latest);
        let c_parts = parse_version_parts(current);
        for (l, c) in l_parts.iter().zip(c_parts.iter()) {
            if l > c {
                return true;
            }
            if l < c {
                return false;
            }
        }
        l_parts.len() > c_parts.len()
    }

    #[tauri::command]
    pub async fn check_for_updates() -> Result<Option<UpdateInfo>, String> {
        let current_version = env!("CARGO_PKG_VERSION");
        let client = Client::builder()
            .user_agent("CiupWifi-App")
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| e.to_string())?;

        let resp = match client
            .get("https://api.github.com/repos/seojihyuk26/CiupWifi/releases/latest")
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => r,
            _ => return Ok(None),
        };

        let text = match resp.text().await {
            Ok(t) => t,
            Err(_) => return Ok(None),
        };

        let json: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => return Ok(None),
        };

        let tag = json.get("tag_name").and_then(|t| t.as_str()).unwrap_or("");
        let latest_version = tag.trim_start_matches('v').to_string();

        if !is_newer_version(&latest_version, current_version) {
            return Ok(None);
        }

        let body = json
            .get("body")
            .and_then(|b| b.as_str())
            .unwrap_or("")
            .to_string();
        let assets = json.get("assets").and_then(|a| a.as_array());

        let mut download_url = String::new();
        let mut asset_name = String::new();

        if let Some(assets) = assets {
            for asset in assets {
                let name = asset.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let url = asset
                    .get("browser_download_url")
                    .and_then(|u| u.as_str())
                    .unwrap_or("");

                #[cfg(target_os = "windows")]
                if name.ends_with(".exe") {
                    download_url = url.to_string();
                    asset_name = name.to_string();
                    break;
                }

                #[cfg(target_os = "macos")]
                if name.ends_with(".dmg") {
                    download_url = url.to_string();
                    asset_name = name.to_string();
                    break;
                }

                #[cfg(target_os = "android")]
                if name.ends_with(".apk") {
                    download_url = url.to_string();
                    asset_name = name.to_string();
                    break;
                }

                #[cfg(not(any(
                    target_os = "windows",
                    target_os = "macos",
                    target_os = "android"
                )))]
                if name.ends_with(".tar.gz")
                    || name.ends_with(".deb")
                    || name.ends_with(".AppImage")
                {
                    download_url = url.to_string();
                    asset_name = name.to_string();
                    break;
                }
            }
        }

        if download_url.is_empty() {
            return Ok(None);
        }

        Ok(Some(UpdateInfo {
            current_version: current_version.to_string(),
            latest_version,
            download_url,
            asset_name,
            release_notes: body,
        }))
    }

    #[tauri::command]
    pub async fn download_and_install_update(
        download_url: String,
        asset_name: String,
    ) -> Result<String, String> {
        let client = Client::builder()
            .user_agent("CiupWifi-App")
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;

        let bytes = client
            .get(&download_url)
            .send()
            .await
            .map_err(|e| format!("Download failed: {}", e))?
            .bytes()
            .await
            .map_err(|e| format!("Reading download data failed: {}", e))?;

        let temp_file = std::env::temp_dir().join(&asset_name);
        std::fs::write(&temp_file, &bytes)
            .map_err(|e| format!("Saving update file failed: {}", e))?;

        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new(&temp_file).spawn();
            std::process::exit(0);
        }

        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("open").arg(&temp_file).spawn();
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = &temp_file;
        }

        Ok(format!("Update downloaded to {:?}", temp_file))
    }
}
