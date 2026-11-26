use reqwest::{Client, Url};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum ProbeResult {
    Online,
    Captive {
        portal_url: String,
        initial_html: Option<String>,
    },
    Offline,
}

#[derive(Debug, Clone)]
pub enum LoginOutcome {
    Success {
        portal_url: Option<String>,
        logout_url: Option<String>,
    },
    AuthFailed(String),
    Error(String),
}

fn build_probe_client() -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(4))
        .redirect(reqwest::redirect::Policy::none())
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())
}

fn build_portal_client() -> Result<Client, String> {
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(12))
        .redirect(reqwest::redirect::Policy::none())
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())
}

pub fn resolve_url(base: &Url, target: &str) -> Option<Url> {
    let cleaned = target
        .replace("&amp;", "&")
        .trim()
        .trim_matches(|c| c == '"' || c == '\'')
        .to_string();
    if cleaned.is_empty() || cleaned.starts_with('#') || cleaned.to_lowercase().starts_with("javascript:") {
        return None;
    }
    base.join(&cleaned).ok()
}

fn is_cloudflare_upgrade(loc: &str) -> bool {
    let lower = loc.to_lowercase();
    lower.starts_with("https://1.1.1.1")
        || lower.contains("cloudflare.com")
        || lower.contains("one.one.one.one")
}

pub fn looks_like_login_form(html: &str) -> bool {
    let lower = html.to_lowercase();
    (lower.contains("<form") && lower.contains("password"))
        || lower.contains("fgtauth")
        || lower.contains("ft_un")
        || lower.contains("ft_pd")
        || (lower.contains("<form") && lower.contains("magic"))
}

pub fn extract_redirect_from_html(html: &str, base_url: &Url) -> Option<String> {
    let lower = html.to_lowercase();

    // 1. <meta http-equiv="refresh" content="0;url=...">
    if let Some(meta_idx) = lower.find("http-equiv") {
        let snippet = &lower[meta_idx..];
        if snippet.starts_with("http-equiv=\"refresh\"")
            || snippet.starts_with("http-equiv='refresh'")
            || snippet.starts_with("http-equiv=refresh")
        {
            if let Some(c_idx) = snippet.find("content=") {
                let after_content = &html[meta_idx + c_idx + 8..];
                let after_lower = after_content.to_lowercase();
                if let Some(u_idx) = after_lower.find("url=") {
                    let url_raw = &after_content[u_idx + 4..];
                    let trimmed = url_raw.trim_start_matches(|c: char| c == ' ' || c == '"' || c == '\'');
                    let end = trimmed
                        .find(|c: char| c == '"' || c == '\'' || c == '>' || c.is_whitespace())
                        .unwrap_or(trimmed.len());
                    if let Some(resolved) = resolve_url(base_url, &trimmed[..end]) {
                        return Some(resolved.to_string());
                    }
                }
            }
        }
    }

    // 2. JS redirects: window.location = "...", location.href = "...", location.replace("...")
    for pat in [
        "location.href=",
        "location.href =",
        "window.location=",
        "window.location =",
        "top.location=",
        "top.location =",
        "location.replace(",
        "location.assign(",
    ] {
        if let Some(idx) = lower.find(pat) {
            let rest = &html[idx + pat.len()..];
            let trimmed = rest.trim_start();
            if let Some(q) = trimmed.chars().next() {
                if q == '"' || q == '\'' {
                    if let Some(end) = trimmed[1..].find(q) {
                        let candidate = &trimmed[1..1 + end];
                        if let Some(resolved) = resolve_url(base_url, candidate) {
                            return Some(resolved.to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

/// Parses the login `<form>` and all `<input>` fields, resolving `action` relative to `base_url`.
/// Returns `(action_url, form_data, detected_user_field, detected_pass_field)`.
pub fn parse_form(
    html: &str,
    base_url: &Url,
) -> (String, HashMap<String, String>, Option<String>, Option<String>) {
    let lower = html.to_lowercase();
    let mut form_data: HashMap<String, String> = HashMap::new();
    let mut action = base_url.to_string();

    // Pick the <form> that contains a password/ft_un input if multiple forms exist
    let mut chosen_form_range: Option<(usize, usize)> = None;
    let mut search_pos = 0;
    while let Some(f_rel) = lower[search_pos..].find("<form") {
        let f_start = search_pos + f_rel;
        let f_end = lower[f_start..]
            .find("</form>")
            .map(|e| f_start + e + 7)
            .unwrap_or(html.len());
        let block = &lower[f_start..f_end];
        if chosen_form_range.is_none()
            || block.contains("password")
            || block.contains("ft_un")
            || block.contains("ft_pd")
        {
            chosen_form_range = Some((f_start, f_end));
            if block.contains("password") || block.contains("ft_un") {
                break;
            }
        }
        search_pos = f_end;
    }

    let (slice_start, slice_end) = chosen_form_range.unwrap_or((0, html.len()));
    let form_html = &html[slice_start..slice_end];
    let form_lower = &lower[slice_start..slice_end];

    if let Some(tag_end) = form_lower.find('>') {
        let open_tag = &form_html[..tag_end];
        if let Some(act) = get_html_attr(open_tag, "action") {
            if !act.trim().is_empty() {
                if let Some(resolved) = resolve_url(base_url, &act) {
                    action = resolved.to_string();
                }
            }
        }
    }

    let mut user_field: Option<String> = None;
    let mut pass_field: Option<String> = None;
    let mut pos = 0;

    while let Some(tag_rel) = form_lower[pos..].find("<input") {
        let abs_start = pos + tag_rel;
        let tag_end = match form_lower[abs_start..].find('>') {
            Some(e) => abs_start + e,
            None => break,
        };
        let tag = &form_html[abs_start..tag_end];
        let name = get_html_attr(tag, "name");
        let value = get_html_attr(tag, "value").unwrap_or_default();
        let input_type = get_html_attr(tag, "type")
            .unwrap_or_else(|| "text".to_string())
            .to_lowercase();
        let id = get_html_attr(tag, "id").unwrap_or_default().to_lowercase();

        if let Some(ref field_name) = name {
            if !field_name.is_empty() {
                let nl = field_name.to_lowercase();
                if input_type != "submit" && input_type != "button" && input_type != "reset" {
                    form_data.insert(field_name.clone(), value);
                }
                if input_type == "password"
                    || id == "ft_pd"
                    || nl.contains("pass")
                    || nl == "ft_pd"
                {
                    pass_field = Some(field_name.clone());
                } else if user_field.is_none()
                    && (id == "ft_un"
                        || nl == "ft_un"
                        || nl.contains("user")
                        || nl.contains("login")
                        || input_type == "text"
                        || input_type == "email")
                {
                    user_field = Some(field_name.clone());
                }
            }
        }
        pos = tag_end + 1;
    }

    (action, form_data, user_field, pass_field)
}

fn get_html_attr(tag: &str, attr_name: &str) -> Option<String> {
    let lower = tag.to_lowercase();
    let target = format!("{}=", attr_name.to_lowercase());
    let mut search_from = 0;
    while let Some(rel) = lower[search_from..].find(&target) {
        let idx = search_from + rel;
        if idx > 0 {
            let prev = lower.as_bytes()[idx - 1] as char;
            if !prev.is_whitespace() && prev != '<' {
                search_from = idx + target.len();
                continue;
            }
        }
        let val_str = tag[idx + target.len()..].trim_start();
        let q = val_str.chars().next()?;
        let raw = if q == '"' || q == '\'' {
            let end = val_str[1..].find(q)?;
            &val_str[1..1 + end]
        } else {
            let end = val_str
                .find(|c: char| c.is_whitespace() || c == '>')
                .unwrap_or(val_str.len());
            &val_str[..end]
        };
        return Some(raw.replace("&amp;", "&"));
    }
    None
}

pub fn extract_logout_url(html: &str, base_url: &Url) -> Option<String> {
    let lower = html.to_lowercase();
    for marker in ["logout?", "/logout"] {
        if let Some(idx) = lower.find(marker) {
            let start = html[..idx]
                .rfind(|c: char| c == '"' || c == '\'' || c.is_whitespace() || c == '(')
                .map(|i| i + 1)
                .unwrap_or(0);
            let end = html[idx..]
                .find(|c: char| c == '"' || c == '\'' || c.is_whitespace() || c == '>' || c == ')')
                .map(|i| idx + i)
                .unwrap_or(html.len());
            if let Some(resolved) = resolve_url(base_url, &html[start..end]) {
                return Some(resolved.to_string());
            }
        }
    }
    None
}

fn extract_magic_token(url_str: &str) -> Option<String> {
    if let Some(idx) = url_str.find("magic=") {
        let val = &url_str[idx + 6..];
        let tok = val.split('&').next().unwrap_or(val);
        if !tok.is_empty() {
            return Some(tok.to_string());
        }
    }
    if let Some(idx) = url_str.find("fgtauth?") {
        let val = &url_str[idx + 8..];
        let tok = val.split('&').next().unwrap_or(val);
        if !tok.is_empty() && !tok.contains('=') {
            return Some(tok.to_string());
        }
    }
    None
}

pub async fn probe_connectivity(gateway_hint: Option<&str>) -> ProbeResult {
    let client = match build_probe_client() {
        Ok(c) => c,
        Err(_) => return ProbeResult::Offline,
    };

    let endpoints = [
        "http://connectivitycheck.gstatic.com/generate_204",
        "http://captive.apple.com/hotspot-detect.html",
        "http://detectportal.firefox.com/canonical.html",
        "http://www.msftconnecttest.com/connecttest.txt",
        "http://neverssl.com/",
        "http://1.1.1.1/",
    ];

    for endpoint in endpoints {
        let resp = match client.get(endpoint).send().await {
            Ok(r) => r,
            Err(_) => continue,
        };

        let status = resp.status().as_u16();
        let final_url = resp.url().clone();

        if status == 204
            && (endpoint.contains("generate_204") || endpoint.contains("gen_204"))
        {
            return ProbeResult::Online;
        }

        if let Some(loc_header) = resp.headers().get(reqwest::header::LOCATION) {
            if let Ok(loc) = loc_header.to_str() {
                if endpoint == "http://1.1.1.1/" && is_cloudflare_upgrade(loc) {
                    return ProbeResult::Online;
                }
                if let Some(resolved) = resolve_url(&final_url, loc) {
                    return ProbeResult::Captive {
                        portal_url: resolved.to_string(),
                        initial_html: None,
                    };
                }
            }
        }

        if status == 200 {
            let text = resp.text().await.unwrap_or_default();
            let lower = text.to_lowercase();

            if endpoint.contains("captive.apple.com")
                && text.contains("Success")
                && !looks_like_login_form(&text)
                && !lower.contains("http-equiv")
            {
                return ProbeResult::Online;
            }
            if endpoint.contains("detectportal.firefox.com")
                && lower.contains("success")
                && !looks_like_login_form(&text)
                && !lower.contains("http-equiv")
            {
                return ProbeResult::Online;
            }
            if endpoint.contains("msftconnecttest.com")
                && text.trim() == "Microsoft Connect Test"
            {
                return ProbeResult::Online;
            }

            if let Some(redir) = extract_redirect_from_html(&text, &final_url) {
                return ProbeResult::Captive {
                    portal_url: redir,
                    initial_html: None,
                };
            }
            if looks_like_login_form(&text) {
                return ProbeResult::Captive {
                    portal_url: final_url.to_string(),
                    initial_html: Some(text),
                };
            }
        }
    }

    // Dynamic gateway fallback across common FortiGate / captive portal ports and schemes
    let mut hosts: Vec<String> = Vec::new();
    if let Some(gw) = gateway_hint {
        let trimmed = gw.trim();
        if !trimmed.is_empty() {
            hosts.push(trimmed.to_string());
        }
    }
    if !hosts.iter().any(|h| h == "10.254.0.254") {
        hosts.push("10.254.0.254".to_string());
    }

    let port_schemes = [
        ("http", ""),
        ("http", ":1000"),
        ("https", ":1003"),
        ("http", ":8000"),
        ("http", ":8080"),
        ("https", ""),
    ];

    for host in hosts {
        for (scheme, port) in port_schemes {
            let url = format!("{}://{}{}/", scheme, host, port);
            if let Ok(resp) = client.get(&url).send().await {
                let resp_url = resp.url().clone();
                if let Some(loc) = resp
                    .headers()
                    .get(reqwest::header::LOCATION)
                    .and_then(|l| l.to_str().ok())
                {
                    if let Some(resolved) = resolve_url(&resp_url, loc) {
                        return ProbeResult::Captive {
                            portal_url: resolved.to_string(),
                            initial_html: None,
                        };
                    }
                }
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    if let Some(redir) = extract_redirect_from_html(&text, &resp_url) {
                        return ProbeResult::Captive {
                            portal_url: redir,
                            initial_html: None,
                        };
                    }
                    if looks_like_login_form(&text) {
                        return ProbeResult::Captive {
                            portal_url: resp_url.to_string(),
                            initial_html: Some(text),
                        };
                    }
                }
            }
        }
    }

    ProbeResult::Offline
}

pub async fn login_to_portal(
    username: &str,
    password: &str,
    gateway_hint: Option<&str>,
) -> LoginOutcome {
    let (initial_url, initial_html) = match probe_connectivity(gateway_hint).await {
        ProbeResult::Online => {
            return LoginOutcome::Success {
                portal_url: None,
                logout_url: None,
            };
        }
        ProbeResult::Offline => {
            return LoginOutcome::Error(
                "Captive portal not detected on WifiCity.".to_string(),
            );
        }
        ProbeResult::Captive {
            portal_url,
            initial_html,
        } => (portal_url, initial_html),
    };

    let client = match build_portal_client() {
        Ok(c) => c,
        Err(e) => return LoginOutcome::Error(e),
    };

    let mut current_url = match Url::parse(&initial_url) {
        Ok(u) => u,
        Err(e) => return LoginOutcome::Error(format!("Invalid portal URL: {}", e)),
    };
    let mut magic_token = extract_magic_token(&initial_url);
    let mut html_text = String::new();

    if let Some(html) = initial_html {
        html_text = html;
    } else {
        for _hop in 0..6 {
            let resp = match client.get(current_url.clone()).send().await {
                Ok(r) => r,
                Err(e) => {
                    return LoginOutcome::Error(format!(
                        "Failed to connect to captive portal: {}",
                        e
                    ))
                }
            };

            current_url = resp.url().clone();
            if magic_token.is_none() {
                magic_token = extract_magic_token(current_url.as_str());
            }

            if let Some(loc) = resp
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|l| l.to_str().ok())
            {
                if let Some(next_url) = resolve_url(&current_url, loc) {
                    current_url = next_url;
                    continue;
                }
            }

            html_text = resp.text().await.unwrap_or_default();
            if looks_like_login_form(&html_text) {
                break;
            }
            if let Some(next) = extract_redirect_from_html(&html_text, &current_url) {
                if let Ok(next_url) = Url::parse(&next) {
                    if next_url != current_url {
                        current_url = next_url;
                        continue;
                    }
                }
            }
            break;
        }
    }

    if magic_token.is_none() {
        magic_token = extract_magic_token(current_url.as_str());
    }

    let (action_url, mut form_data, user_field, pass_field) =
        parse_form(&html_text, &current_url);

    if let Some(ref tok) = magic_token {
        let missing_magic = form_data
            .get("magic")
            .map(|v| v.is_empty())
            .unwrap_or(true);
        if missing_magic {
            form_data.insert("magic".to_string(), tok.clone());
        }
    }

    if let Some(ref uf) = user_field {
        form_data.insert(uf.clone(), username.to_string());
    }
    if let Some(ref pf) = pass_field {
        form_data.insert(pf.clone(), password.to_string());
    }
    if user_field.is_none() && pass_field.is_none() {
        form_data.insert("ft_un".to_string(), username.to_string());
        form_data.insert("ft_pd".to_string(), password.to_string());
        form_data.insert("username".to_string(), username.to_string());
        form_data.insert("password".to_string(), password.to_string());
    } else {
        if form_data.contains_key("ft_un") {
            form_data.insert("ft_un".to_string(), username.to_string());
        }
        if form_data.contains_key("ft_pd") {
            form_data.insert("ft_pd".to_string(), password.to_string());
        }
    }

    let post_resp = match client
        .post(&action_url)
        .header(reqwest::header::REFERER, current_url.as_str())
        .form(&form_data)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return LoginOutcome::Error(format!("Login request failed: {}", e)),
    };

    // Follow post-login redirects (301/302/303 -> GET) manually up to 5 hops
    let mut final_post_url = post_resp.url().clone();
    let mut post_body = String::new();
    let mut resp_opt = Some(post_resp);

    for _ in 0..5 {
        let r = match resp_opt.take() {
            Some(x) => x,
            None => break,
        };
        final_post_url = r.url().clone();
        let status = r.status();
        if status.is_redirection() {
            if let Some(loc) = r
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|l| l.to_str().ok())
            {
                if let Some(next_url) = resolve_url(&final_post_url, loc) {
                    if let Ok(next_resp) = client
                        .get(next_url)
                        .header(reqwest::header::REFERER, final_post_url.as_str())
                        .send()
                        .await
                    {
                        resp_opt = Some(next_resp);
                        continue;
                    }
                }
            }
        }
        post_body = r.text().await.unwrap_or_default();
        break;
    }

    let mut logout_url = extract_logout_url(&post_body, &final_post_url);
    if logout_url.is_none() {
        if let Some(ref tok) = magic_token {
            if final_post_url.as_str().to_lowercase().contains("keepalive") {
                if let Ok(base) = current_url.join(&format!("/logout?{}", tok)) {
                    logout_url = Some(base.to_string());
                }
            }
        }
    }

    let lower_body = post_body.to_lowercase();
    let explicit_auth_fail = lower_body.contains("authentication failed")
        || lower_body.contains("invalid username")
        || lower_body.contains("invalid password")
        || lower_body.contains("wrong password")
        || lower_body.contains("login failed")
        || lower_body.contains("failed to authenticate")
        || lower_body.contains("incorrect username")
        || lower_body.contains("incorrect password");

    let explicit_success_hint = lower_body.contains("keepalive")
        || lower_body.contains("keep this window open")
        || lower_body.contains("authentication succeeded")
        || lower_body.contains("welcome")
        || post_body.contains("Success");

    tokio::time::sleep(Duration::from_millis(700)).await;

    match probe_connectivity(gateway_hint).await {
        ProbeResult::Online => LoginOutcome::Success {
            portal_url: Some(current_url.to_string()),
            logout_url,
        },
        _ => {
            if explicit_auth_fail {
                return LoginOutcome::AuthFailed(
                    "Invalid campus username or password.".to_string(),
                );
            }
            if explicit_success_hint {
                tokio::time::sleep(Duration::from_millis(800)).await;
                if matches!(probe_connectivity(gateway_hint).await, ProbeResult::Online) {
                    return LoginOutcome::Success {
                        portal_url: Some(current_url.to_string()),
                        logout_url,
                    };
                }
            }
            if looks_like_login_form(&post_body) {
                LoginOutcome::AuthFailed(
                    "Portal rejected login. Please verify your username and password.".to_string(),
                )
            } else {
                LoginOutcome::Error(
                    "Submitted portal login, waiting for network connectivity.".to_string(),
                )
            }
        }
    }
}

pub async fn logout_from_portal(logout_url: &str) {
    if let Ok(client) = build_probe_client() {
        let _ = client.get(logout_url).send().await;
    }
}

pub fn is_ciup_portal(
    gateway: Option<&str>,
    portal_url: Option<&str>,
    html: Option<&str>,
    known_gateways: &[String],
) -> bool {
    if let Some(gw) = gateway {
        let clean = gw.trim();
        if clean.starts_with("10.254.") || known_gateways.iter().any(|k| k == clean) {
            return true;
        }
    }
    if let Some(u) = portal_url {
        let lower = u.to_lowercase();
        if lower.contains("10.254.")
            || lower.contains("fgtauth")
            || lower.contains("wificity")
            || lower.contains("ciup")
        {
            return true;
        }
    }
    if let Some(h) = html {
        let lower = h.to_lowercase();
        if lower.contains("ft_un") && lower.contains("ft_pd") {
            return true;
        }
    }
    false
}
