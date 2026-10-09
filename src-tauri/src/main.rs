#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    ciupwifi_lib::run()
}
