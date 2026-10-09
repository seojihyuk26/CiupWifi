# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Automatic login and session maintenance utility for the "WifiCity" network at Cité internationale universitaire de Paris (CIUP).</b>
</p>

<p align="center">
  <a href="README.md"><b>English</b></a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.de.md">Deutsch</a>
</p>

> [!WARNING]
> **Disclaimer**: This is an independent, community-developed open-source tool. It is **not** an official application and has no affiliation with or endorsement from the administration or IT services of the Cité internationale universitaire de Paris (CIUP).

---

## Overview

The campus Wi-Fi network at CIUP (**WifiCity**) enforces periodic captive portal session timeouts, requiring residents to repeatedly reopen a web browser and re-authenticate throughout the day.

**CiupWifi** automates this workflow:
- Authenticates in the background without requiring a browser window.
- Tracks natural session durations and proactively renews the connection before the timeout occurs.
- Minimizes to the system tray with negligible memory usage (< 15 MB).

---

## Usage

1. Connect your device to the **WifiCity** Wi-Fi network.
2. Launch **CiupWifi**, enter your campus credentials, and click **Connect**.
3. Close the window. The application continues running in the background/system tray and handles all subsequent logins and renewals automatically.

---

## Downloads

Official binary packages are available on the **[Releases](https://github.com/seojihyuk26/CiupWifi/releases/latest)** page:

| Platform | Package | Notes |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (or `.msi`) | Standard Windows installer. Runs from Start menu. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Drag `CiupWifi.app` to Applications.<br>*(On Gatekeeper prompt: right-click > **Open**)* |
| **Android** | `app-universal-release-unsigned.apk` | Standalone APK package for Android devices. |
| **iOS / iPadOS** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Run via [Orion Browser](https://kagi.com/orion/) or [Userscripts for Safari](https://apps.apple.com/app/userscripts/id1463298887). |

---

## Privacy & Security

- **Local-Only Storage**: Credentials are saved exclusively on your local device in standard application data storage.
- **Direct Gateway Communication**: Network requests are sent solely to the local captive portal interface (`10.254.0.254`).
- **No Telemetry**: No tracking, analytics, or external servers are involved.
- **Open Source**: Full source code is available for auditing.

---

## Technical Details & Architecture

For developers and curious users, here is how CiupWifi operates under the hood:

### 1. Direct Native HTTP Authentication
Rather than injecting JavaScript into an in-app WebView or browser, CiupWifi utilizes a lightweight Rust backend via `reqwest`:
- Queries `http://www.google.com/gen_204` to probe for captive network redirection.
- Intercepts the HTTP 302 redirection to FortiGate portal endpoints (`http://10.254.0.254:1000/fgtauth?...`).
- Extracts the dynamic session token (`magic`) and submits dual-compatible form payloads (`ft_un`/`username`, `ft_pd`/`password`, `magic`).
- Verifies successful connectivity via subsequent HTTP 204 responses.

### 2. Proactive Session Renewal
- Filters out transient noise (such as device sleep or signal drops) by only recording **sessions that lasted at least 1 hour** (retaining the last 3 records).
- Uses the **minimum duration** among these records as a benchmark and automatically renews connection 5 minutes before that deadline.
- Proactively renewed sessions are excluded from natural expiration history to keep timing accurate.

---

## References & Credits

This project builds upon reverse-engineering and automation work from the CIUP student developer community:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: Identified the FortiGate authentication port `1000`, the `magic` token extraction mechanism, and provided initial Windows and Linux CLI scripts.
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: The original Linux shell script prototype for automated WifiCity portal login.

---

## Building from Source

### Prerequisites
- Node.js 20+
- Rust 1.77+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev

# Build release packages
npm run build
```

---

## License

Distributed under the [MIT License](LICENSE).
