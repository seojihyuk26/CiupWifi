#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--silent" || a == "-s" || a == "--headless" || a == "--login") {
        ciupwifi_lib::run_headless();
    } else {
        ciupwifi_lib::run();
    }
}
