use std::path::PathBuf;

pub fn get_app_dir(app: Option<&tauri::AppHandle>) -> PathBuf {
    use tauri::Manager;

    if let Some(a) = app {
        if let Ok(dir) = a.path().app_data_dir() {
            return dir;
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata).join("org.ciup.wifi");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join("Library/Application Support/org.ciup.wifi");
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join("org.ciup.wifi");
        } else if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config/org.ciup.wifi");
        }
    }
    PathBuf::from("org.ciup.wifi")
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn load_credentials_direct(app: Option<&tauri::AppHandle>) -> Option<Credentials> {
    let path = get_app_dir(app).join("credentials.json");
    let content = std::fs::read_to_string(path).ok()?;
    let creds: Credentials = serde_json::from_str(&content).ok()?;
    if creds.username.is_empty() || creds.password.is_empty() {
        None
    } else {
        Some(creds)
    }
}

pub fn save_credentials_direct(
    app: Option<&tauri::AppHandle>,
    username: &str,
    password: &str,
) -> Result<(), String> {
    let dir = get_app_dir(app);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let payload = serde_json::json!({
        "username": username,
        "password": password,
    });
    let path = dir.join("credentials.json");
    let tmp = dir.join("credentials.json.tmp");
    std::fs::write(&tmp, payload.to_string()).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, &path).or_else(|_| std::fs::write(&path, payload.to_string())).map_err(|e| e.to_string())?;
    Ok(())
}

fn default_true() -> bool {
    true
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AppState {
    #[serde(default = "default_true")]
    pub auto_enabled: bool,
    #[serde(default)]
    pub user_paused: bool,
    #[serde(default)]
    pub was_on_wificity: bool,
    #[serde(default)]
    pub last_logout_url: Option<String>,
    #[serde(default)]
    pub last_portal_url: Option<String>,
    #[serde(default)]
    pub auth_failed: bool,
    #[serde(default)]
    pub last_error: Option<String>,
    #[serde(default)]
    pub known_ciup_gateways: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            auto_enabled: true,
            user_paused: false,
            was_on_wificity: false,
            last_logout_url: None,
            last_portal_url: None,
            auth_failed: false,
            last_error: None,
            known_ciup_gateways: Vec::new(),
        }
    }
}

impl AppState {
    pub fn record_ciup_gateway(&mut self, gw: Option<&str>) {
        if let Some(raw) = gw {
            let clean = raw.trim();
            if !clean.is_empty() && !self.known_ciup_gateways.iter().any(|g| g == clean) {
                self.known_ciup_gateways.push(clean.to_string());
                if self.known_ciup_gateways.len() > 8 {
                    self.known_ciup_gateways.remove(0);
                }
            }
        }
    }
}

pub fn load_state(app: Option<&tauri::AppHandle>) -> AppState {
    let path = get_app_dir(app).join("state.json");
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppState::default(),
    }
}

pub fn save_state(app: Option<&tauri::AppHandle>, state: &AppState) -> Result<(), String> {
    let dir = get_app_dir(app);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("state.json");
    let tmp = dir.join("state.json.tmp");
    let data = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    std::fs::write(&tmp, &data).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, &path).or_else(|_| std::fs::write(&path, &data)).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct NetworkStatus {
    /// "wifi_city_online" | "wifi_city_captive" | "not_on_wificity" | "disconnected" | "paused" | "permission_needed"
    pub state: String,
    pub ssid: Option<String>,
    pub is_wifi: bool,
    pub is_cellular: bool,
    pub is_wificity: bool,
    pub user_paused: bool,
    pub auto_enabled: bool,
    pub auth_failed: bool,
    pub portal_url: Option<String>,
    pub detail: Option<String>,
}
