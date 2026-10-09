use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            crate::commands::login_to_portal,
            crate::commands::check_connectivity,
            crate::commands::save_credentials,
            crate::commands::load_credentials,
            crate::commands::save_session_history,
            crate::commands::load_session_history,
        ])
        .setup(|app| {
            // 시스템 트레이 설정 (데스크탑만)
            #[cfg(desktop)]
            {
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
                use tauri::menu::{Menu, MenuItem};

                let quit = MenuItem::with_id(app, "quit", "Quit CiupWifi", true, None::<&str>)?;
                let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show, &quit])?;

                TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "quit" => app.exit(0),
                        "show" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                    })
                    .build(app)?;
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // 창 닫기 → 트레이로 최소화 (데스크탑만)
            #[cfg(desktop)]
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod commands {
    use reqwest::Client;
    use scraper::{Html, Selector};
    use std::collections::HashMap;
    use tauri::Manager;

    // ── 포털 직접 HTTP 로그인 ──────────────────────────────────────────────────
    #[tauri::command]
    pub async fn login_to_portal(username: String, password: String) -> Result<bool, String> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;

        // 1. GET 포털 페이지 → form action + hidden fields 추출
        let resp = client
            .get("http://10.254.0.254/")
            .send()
            .await
            .map_err(|e| format!("포털 연결 실패: {}", e))?;

        let html_text = resp.text().await.map_err(|e| e.to_string())?;
        let document = Html::parse_document(&html_text);

        // form action 파싱
        let form_selector = Selector::parse("form").unwrap();
        let action = document
            .select(&form_selector)
            .next()
            .and_then(|f| f.value().attr("action"))
            .map(|a| {
                if a.starts_with("http") {
                    a.to_string()
                } else {
                    format!("http://10.254.0.254{}", a)
                }
            })
            .unwrap_or_else(|| "http://10.254.0.254/login".to_string());

        // hidden fields 파싱
        let mut form_data: HashMap<String, String> = HashMap::new();
        let hidden_selector = Selector::parse("input[type='hidden']").unwrap();
        for input in document.select(&hidden_selector) {
            if let (Some(name), Some(value)) = (
                input.value().attr("name"),
                input.value().attr("value"),
            ) {
                form_data.insert(name.to_string(), value.to_string());
            }
        }

        // 자격증명 추가 (포털 필드명: ft_un, ft_pd)
        form_data.insert("ft_un".to_string(), username);
        form_data.insert("ft_pd".to_string(), password);

        // 2. POST → 로그인
        let login_resp = client
            .post(&action)
            .form(&form_data)
            .send()
            .await
            .map_err(|e| format!("로그인 요청 실패: {}", e))?;

        let body = login_resp.text().await.unwrap_or_default();

        // 3. 성공 판정: "Success" 포함 여부
        let success = body.contains("Success")
            || body.to_lowercase().contains("welcome")
            || body.to_lowercase().contains("connected");

        Ok(success)
    }

    // ── 인터넷 연결 / 캡티브 포털 감지 ──────────────────────────────────────
    #[tauri::command]
    pub async fn check_connectivity() -> Result<String, String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| e.to_string())?;

        // Apple connectivity check (리다이렉트 없으면 "Success" 반환)
        match client.get("http://captive.apple.com/hotspot-detect.html").send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body = resp.text().await.unwrap_or_default();
                if status == 200 && body.contains("Success") {
                    Ok("connected".to_string())
                } else {
                    Ok("captive".to_string())
                }
            }
            Err(_) => Ok("captive".to_string()),
        }
    }

    // ── 자격증명 저장 / 불러오기 (store 플러그인 대신 간단히 앱 데이터 폴더) ──
    #[tauri::command]
    pub fn save_credentials(
        app: tauri::AppHandle,
        username: String,
        password: String,
    ) -> Result<(), String> {
        use std::io::Write;
        let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        let cred = serde_json::json!({ "username": username, "password": password });
        let mut f = std::fs::File::create(data_dir.join("credentials.json"))
            .map_err(|e| e.to_string())?;
        f.write_all(cred.to_string().as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub fn load_credentials(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
        let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let path = data_dir.join("credentials.json");
        if !path.exists() {
            return Ok(serde_json::json!({ "username": "", "password": "" }));
        }
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    // ── 세션 히스토리 저장 / 불러오기 ─────────────────────────────────────────
    #[tauri::command]
    pub fn save_session_history(
        app: tauri::AppHandle,
        history: Vec<u64>,
    ) -> Result<(), String> {
        use std::io::Write;
        let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
        let mut f = std::fs::File::create(data_dir.join("session_history.json"))
            .map_err(|e| e.to_string())?;
        f.write_all(serde_json::to_string(&history).unwrap().as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    #[tauri::command]
    pub fn load_session_history(app: tauri::AppHandle) -> Result<Vec<u64>, String> {
        let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
        let path = data_dir.join("session_history.json");
        if !path.exists() {
            return Ok(vec![]);
        }
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }
}
