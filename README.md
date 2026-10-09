# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Fast, automatic login & proactive session reconnection for CIUP (Cité internationale universitaire de Paris) Wi-Fi.</b>
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
> **Disclaimer**: This is an **unofficial, student-made open-source tool**. It is **NOT** developed, approved, or affiliated in any way with the administration or IT services of the **Cité internationale universitaire de Paris (CIUP)**. Use it at your own discretion.

---

## 💡 What is CiupWifi?

At the Cité internationale universitaire de Paris (CIUP), the captive portal regularly expires sessions, forcing residents to manually log in over and over throughout the day.

**CiupWifi** is a native cross-platform application (Windows, macOS, Android) built with **Tauri v2** that automates this entire process:
- **Zero Browser Required**: Logs into `10.254.0.254` via direct native HTTP requests — no annoying browser windows or WebViews.
- **Proactive Relogin**: Analyzes your past session durations and predicts expiration, automatically reconnecting minutes *before* your Wi-Fi drops.
- **Lightweight & Battery Efficient**: Sits unobtrusively in your system tray or background service consuming minimal memory (<15 MB).
- **Privacy First**: Credentials are stored strictly on your local device and never leave your machine.

---

## 📥 Download & Installation

Visit the **[Latest GitHub Releases](https://github.com/seojihyuk/CiupWifi/releases/latest)** to download the version for your device:

| Platform | Download | Instructions |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (or `.msi`) | Run installer and launch from Start menu. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Open `.dmg` and drag `CiupWifi.app` to Applications.<br>*(If prompted about an unidentified developer, right-click and choose **Open**)* |
| **Android** | `app-universal-release-unsigned.apk` | Download and install APK. Allow installation from unknown sources if prompted. |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Use [Orion Browser](https://kagi.com/orion/) or [Userscripts for Safari](https://apps.apple.com/app/userscripts/id1463298887) and install `wifiLogin.js`. |

---

## 🚀 How to Use

1. **Connect** to the CIUP Wi-Fi network.
2. **Launch CiupWifi**.
3. Enter your campus Wi-Fi **Username** and **Password** once, then click **Connect**.
4. That's it! 
   - On Desktop: Close the window to minimize to the system tray.
   - The app will automatically reconnect whenever your session is about to expire or when you rejoin the network.

---

## 🔒 Security & Privacy

- **No Remote Servers**: CiupWifi does not collect analytics or transmit data to third parties.
- **Local Credentials**: Login credentials are saved locally in your system's app data directory.
- **Open Source**: All source code is completely open for review.

---

## 🛠 For Developers

### Prerequisites
- Node.js 20+
- Rust 1.77+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)

### Run Locally
```bash
npm install
npm run dev
```

### Build Releases
```bash
npm run build
```

---

## 📄 License & Legal Notice

Distributed under the [MIT License](LICENSE).  
**Not an official app**: CiupWifi is an independent open-source utility created by residents for residents, and is not affiliated with, endorsed by, or associated with the Cité internationale universitaire de Paris (CIUP).
