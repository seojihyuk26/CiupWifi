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

    fn parse_form(html_text: &str) -> (String, HashMap<String, String>) {
        let document = Html::parse_document(html_text);
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
        (action, form_data)
    }

    // ── 포털 직접 HTTP 로그인 (WifiCity FortiGate 호환) ────────────────────────
    #[tauri::command]
    pub async fn login_to_portal(username: String, password: String) -> Result<bool, String> {
        let client = Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .map_err(|e| e.to_string())?;

        // 1. WifiCity(FortiGate) 포털 감지 및 URL 확인 (기본 포트: 1000)
        let mut target_url = "http://10.254.0.254:1000/".to_string();
        let probe_client = Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .ok();

        if let Some(probe) = probe_client {
            if let Ok(resp) = probe.get("http://www.google.com/gen_204").send().await {
                if let Some(loc) = resp.headers().get("location").and_then(|l| l.to_str().ok()) {
                    if loc.contains("10.254.0.254") {
                        target_url = loc.to_string();
                    }
                }
            }
        }

        // 2. 포털 페이지 GET 요청 (1000 포트 실패 시 80 포트로 폴백)
        let resp = match client.get(&target_url).send().await {
            Ok(r) => r,
            Err(_) => client
                .get("http://10.254.0.254/")
                .send()
                .await
                .map_err(|e| format!("WifiCity 포털 연결 실패: {}", e))?,
        };

        let current_url = resp.url().to_string();
        let html_text = resp.text().await.map_err(|e| e.to_string())?;
        let (mut action, mut form_data) = parse_form(&html_text);

        if !action.starts_with("http") {
            action = format!("http://10.254.0.254:1000{}", action);
        }

        // URL 쿼리에 magic 파라미터가 있으면 자동 추출 (FortiGate 인증 토큰)
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

        // 자격증명: FortiGate 필드명(ft_un/ft_pd)과 스크립트 표준 필드명(username/password) 모두 전송
        form_data.insert("ft_un".to_string(), username.clone());
        form_data.insert("ft_pd".to_string(), password.clone());
        form_data.insert("username".to_string(), username);
        form_data.insert("password".to_string(), password);

        // 3. POST → 로그인 제출
        let login_resp = client
            .post(&action)
            .form(&form_data)
            .send()
            .await
            .map_err(|e| format!("WifiCity 로그인 요청 실패: {}", e))?;

        let body = login_resp.text().await.unwrap_or_default();

        // 4. 성공 판정
        let mut success = body.contains("Success")
            || body.to_lowercase().contains("welcome")
            || body.to_lowercase().contains("connected")
            || body.to_lowercase().contains("keep this window open");

        // 응답 본문에서 확정할 수 없는 경우 google gen_204로 실제 연결 성공 여부 교차 검증
        if !success {
            if let Ok(verify) = client.get("http://www.google.com/gen_204").send().await {
                if verify.status().as_u16() == 204 {
                    success = true;
                }
            }
        }

        Ok(success)
    }

    // ── 인터넷 연결 / 캡티브 포털 감지 ──────────────────────────────────────
    #[tauri::command]
    pub async fn check_connectivity() -> Result<String, String> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| e.to_string())?;

        // 1. Google 204 검사 (WifiCity 표준 캡티브 프로브)
        if let Ok(resp) = client.get("http://www.google.com/gen_204").send().await {
            if resp.status().as_u16() == 204 {
                return Ok("connected".to_string());
            }
        }

        // 2. Apple hotspot-detect 검사 (보조 검증)
        if let Ok(resp) = client.get("http://captive.apple.com/hotspot-detect.html").send().await {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            if status == 200 && body.contains("Success") {
                return Ok("connected".to_string());
            }
        }

        Ok("captive".to_string())
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
