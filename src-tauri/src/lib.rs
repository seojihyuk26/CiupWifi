use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::commands::login_to_portal,
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

pub fn run_headless() {
    tauri::async_runtime::block_on(async {
        let (username, password) = match read_credentials_direct() {
            Some(creds) => creds,
            None => {
                eprintln!("[CiupWifi] No saved credentials found. Please open CiupWifi once to save your login.");
                return;
            }
        };

        // Check if already connected
        if let Ok(status) = commands::check_connectivity_internal().await {
            if status == "connected" {
                println!("[CiupWifi] Internet is already active.");
                return;
            }
        }

        println!("[CiupWifi] Captive portal detected. Logging in as {}...", username);
        match commands::login_to_portal_internal(&username, &password).await {
            Ok(true) => println!("[CiupWifi] Login successful!"),
            Ok(false) => eprintln!("[CiupWifi] Login rejected. Please verify credentials."),
            Err(e) => eprintln!("[CiupWifi] Connection error: {}", e),
        }
    });
}

fn get_app_dir(app: Option<&tauri::AppHandle>) -> std::path::PathBuf {
    if let Some(a) = app {
        if let Ok(dir) = a.path().app_data_dir() {
            return dir;
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return std::path::PathBuf::from(appdata).join("org.ciup.wifi");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return std::path::PathBuf::from(home).join("Library/Application Support/org.ciup.wifi");
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return std::path::PathBuf::from(xdg).join("org.ciup.wifi");
        } else if let Ok(home) = std::env::var("HOME") {
            return std::path::PathBuf::from(home).join(".config/org.ciup.wifi");
        }
    }
    std::path::PathBuf::from("org.ciup.wifi")
}

pub fn read_credentials_direct() -> Option<(String, String)> {
    let dir = get_app_dir(None);
    let path = dir.join("credentials.json");
    let content = std::fs::read_to_string(path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&content).ok()?;
    let u = v.get("username")?.as_str()?.to_string();
    let p = v.get("password")?.as_str()?.to_string();
    if u.is_empty() || p.is_empty() {
        None
    } else {
        Some((u, p))
    }
}

pub mod commands {
    use reqwest::Client;
    use std::collections::HashMap;

    fn parse_form(html_text: &str) -> (String, HashMap<String, String>) {
        let mut form_data: HashMap<String, String> = HashMap::new();
        let mut action = "http://10.254.0.254:1000/login".to_string();

        if let Some(form_idx) = html_text.to_lowercase().find("<form") {
            let rest = &html_text[form_idx..];
            if let Some(action_idx) = rest.to_lowercase().find("action=") {
                let after = &rest[action_idx + 7..];
                let quote = after.chars().next().unwrap_or('"');
                let (offset, end) = if quote == '"' || quote == '\'' {
                    (1, after[1..].find(quote).unwrap_or(after.len() - 1))
                } else {
                    (0, after.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(after.len()))
                };
                let act = &after[offset..offset + end];
                if !act.is_empty() {
                    action = if act.starts_with("http") {
                        act.to_string()
                    } else {
                        format!("http://10.254.0.254:1000{}", act)
                    };
                }
            }
        }

        let lower = html_text.to_lowercase();
        let mut pos = 0;
        while let Some(tag_start) = lower[pos..].find("<input") {
            let abs_start = pos + tag_start;
            let tag_end = match lower[abs_start..].find('>') {
                Some(e) => abs_start + e,
                None => break,
            };
            let tag = &html_text[abs_start..tag_end];
            let tag_lower = &lower[abs_start..tag_end];

            if tag_lower.contains("hidden") {
                let get_attr = |attr_name: &str| -> Option<String> {
                    let pat = format!("{}=", attr_name);
                    let idx = tag_lower.find(&pat)?;
                    let val_str = &tag[idx + pat.len()..];
                    let q = val_str.chars().next()?;
                    let (off, len) = if q == '"' || q == '\'' {
                        (1, val_str[1..].find(q)?)
                    } else {
                        (0, val_str.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(val_str.len()))
                    };
                    Some(val_str[off..off + len].to_string())
                };

                if let (Some(name), Some(val)) = (get_attr("name"), get_attr("value")) {
                    form_data.insert(name, val);
                }
            }
            pos = tag_end + 1;
        }

        (action, form_data)
    }

    pub async fn login_to_portal_internal(username: &str, password: &str) -> Result<bool, String> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .map_err(|e| e.to_string())?;

        let mut target_url = "http://10.254.0.254:1000/".to_string();
        let probe = Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .ok();

        if let Some(p) = probe {
            if let Ok(resp) = p.get("http://www.google.com/gen_204").send().await {
                if let Some(loc) = resp.headers().get("location").and_then(|l| l.to_str().ok()) {
                    if loc.contains("10.254.0.254") {
                        target_url = loc.to_string();
                    }
                }
            }
        }

        let resp = match client.get(&target_url).send().await {
            Ok(r) => r,
            Err(_) => client
                .get("http://10.254.0.254/")
                .send()
                .await
                .map_err(|e| format!("WifiCity connection failed: {}", e))?,
        };

        let current_url = resp.url().to_string();
        let html_text = resp.text().await.map_err(|e| e.to_string())?;
        let (mut action, mut form_data) = parse_form(&html_text);

        if !action.starts_with("http") {
            action = format!("http://10.254.0.254:1000{}", action);
        }

        for url_str in [&current_url, &target_url] {
            if let Some(idx) = url_str.find("magic=") {
                let val = &url_str[idx + 6..];
                let token = val.split('&').next().unwrap_or(val);
                form_data.insert("magic".to_string(), token.to_string());
                break;
            } else if let Some(idx) = url_str.find("fgtauth?") {
                let val = &url_str[idx + 8..];
                let token = val.split('&').next().unwrap_or(val);
                form_data.insert("magic".to_string(), token.to_string());
                break;
            }
        }

        form_data.insert("ft_un".to_string(), username.to_string());
        form_data.insert("ft_pd".to_string(), password.to_string());
        form_data.insert("username".to_string(), username.to_string());
        form_data.insert("password".to_string(), password.to_string());

        let login_resp = client
            .post(&action)
            .form(&form_data)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;

        let body = login_resp.text().await.unwrap_or_default();
        let mut success = body.contains("Success")
            || body.to_lowercase().contains("welcome")
            || body.to_lowercase().contains("connected")
            || body.to_lowercase().contains("keep this window open");

        if !success {
            if let Ok(verify) = client.get("http://www.google.com/gen_204").send().await {
                if verify.status().as_u16() == 204 {
                    success = true;
                }
            }
        }

        Ok(success)
    }

    pub async fn check_connectivity_internal() -> Result<String, String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| e.to_string())?;

        if let Ok(resp) = client.get("http://www.google.com/gen_204").send().await {
            if resp.status().as_u16() == 204 {
                return Ok("connected".to_string());
            }
        }

        if let Ok(resp) = client.get("http://captive.apple.com/hotspot-detect.html").send().await {
            if resp.status().as_u16() == 200 && resp.text().await.unwrap_or_default().contains("Success") {
                return Ok("connected".to_string());
            }
        }

        Ok("captive".to_string())
    }

    #[tauri::command]
    pub async fn login_to_portal(username: String, password: String) -> Result<bool, String> {
        login_to_portal_internal(&username, &password).await
    }

    #[tauri::command]
    pub async fn check_connectivity() -> Result<String, String> {
        check_connectivity_internal().await
    }

    #[tauri::command]
    pub fn save_credentials(
        app: tauri::AppHandle,
        username: String,
        password: String,
    ) -> Result<(), String> {
        use std::io::Write;
        let data_dir = super::get_app_dir(Some(&app));
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        let cred = serde_json::json!({ "username": username, "password": password });
        let mut f = std::fs::File::create(data_dir.join("credentials.json")).map_err(|e| e.to_string())?;
        f.write_all(cred.to_string().as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub fn load_credentials(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
        let data_dir = super::get_app_dir(Some(&app));
        let path = data_dir.join("credentials.json");
        if !path.exists() {
            return Ok(serde_json::json!({ "username": "", "password": "" }));
        }
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    #[tauri::command]
    pub fn setup_background_task(_enable: bool) -> Result<(), String> {
        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let _exe_path = current_exe.to_string_lossy().to_string();

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            if _enable {
                let _ = std::process::Command::new("schtasks")
                    .args(&["/create", "/tn", "CiupWifiAuto", "/tr", &format!("\"{}\" --silent", _exe_path), "/sc", "minute", "/mo", "30", "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
                let _ = std::process::Command::new("schtasks")
                    .args(&["/create", "/tn", "CiupWifiLogon", "/tr", &format!("\"{}\" --silent", _exe_path), "/sc", "onlogon", "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            } else {
                let _ = std::process::Command::new("schtasks")
                    .args(&["/delete", "/tn", "CiupWifiAuto", "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
                let _ = std::process::Command::new("schtasks")
                    .args(&["/delete", "/tn", "CiupWifiLogon", "/f"])
                    .creation_flags(CREATE_NO_WINDOW)
                    .output();
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = std::env::var("HOME") {
                let plist_path = std::path::PathBuf::from(home)
                    .join("Library/LaunchAgents/org.ciup.wifi.plist");
                if _enable {
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
        <string>--silent</string>
    </array>
    <key>StartInterval</key>
    <integer>1800</integer>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
                        _exe_path
                    );
                    let _ = std::fs::write(&plist_path, plist_content);
                } else {
                    let _ = std::fs::remove_file(&plist_path);
                }
            }
        }

        Ok(())
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
            if l > c { return true; }
            if l < c { return false; }
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

        let body = json.get("body").and_then(|b| b.as_str()).unwrap_or("").to_string();
        let assets = json.get("assets").and_then(|a| a.as_array());

        let mut download_url = String::new();
        let mut asset_name = String::new();

        if let Some(assets) = assets {
            for asset in assets {
                let name = asset.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let url = asset.get("browser_download_url").and_then(|u| u.as_str()).unwrap_or("");

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

                #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "android")))]
                if name.ends_with(".tar.gz") || name.ends_with(".deb") || name.ends_with(".AppImage") {
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
    pub async fn download_and_install_update(download_url: String, asset_name: String) -> Result<String, String> {
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
