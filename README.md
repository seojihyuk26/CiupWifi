# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Auto-login & session renewal utility for CIUP "WifiCity" campus Wi-Fi network.</b><br>
  <i>Stay connected without having to manually log in every time the captive portal expires.</i>
</p>

<p align="center">
  <a href="README.md"><b>English</b></a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Unofficial App**: This is an independent open-source utility developed by a student resident. It is not affiliated with the CIUP administration or IT department.

---

## 📥 Downloads (Latest Release)

Download packages directly from the **[GitHub Releases](https://github.com/seojihyuk26/CiupWifi/releases/latest)** page:

| Platform | Installer | Setup Notes |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Standard setup. (If SmartScreen appears: **More info** → **Run anyway**) |
| **macOS** | `CiupWifi_*_universal.dmg` | Drag to `Applications`. (If blocked: **System Settings** → **Security** → **Open Anyway**) |
| **Android** | `CiupWifi.apk` | Standalone signed APK for Android devices. |
| **iOS / iPadOS / Mac** | [**Greasy Fork Userscript**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Install in Safari via [Userscripts](https://apps.apple.com/app/userscripts/id1463298887) or Orion Browser in 1 second. |

---

## 🚀 Quick Start (30 Seconds)

1. Connect your device to the campus Wi-Fi network **WifiCity**.
2. Launch **CiupWifi**, enter your campus **Username & Password**, and click **Connect**.
3. **You can close the window immediately (`[X]`).**
   - Background re-authentication is automatically registered with your OS scheduler (Windows / macOS).
   - Runs headlessly in ~0.3s only when needed, consuming **virtually 0 MB RAM** when idle.

---

## 🛡️ Security Warnings on First Launch (Windows / macOS / Android)

As an independent open-source utility without enterprise corporate certificates, operating systems may show an unrecognized developer prompt on first launch. You can safely proceed in 1 second using the official procedure for your OS:

* **🪟 Windows (SmartScreen)**:
  - If *"Windows protected your PC"* appears: click **[More info]** → **[Run anyway]**.
* **🍏 macOS (Gatekeeper)**:
  - If blocked: click **[Cancel]** → open Mac **[System Settings]** → **[Privacy & Security]** → scroll down to Security and click **[Open Anyway]** → authenticate.
* **🤖 Android (Google Play Protect)**:
  - If blocked: tap **[More details]** (∨) → tap **[Install anyway]** (or enable *Install unknown apps* for your browser).
* **🌐 Zero-Install Alternative (Browser Userscript)**:
  - If you prefer not to configure system security exceptions, use our [**Greasy Fork Userscript**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) in Safari, Chrome, or Firefox. It runs 100% inside your browser sandbox with zero permissions.

---

## 🔒 Privacy & Security

* **100% Local Storage**: Your credentials are stored exclusively on your device's local storage.
* **Direct Gateway Only**: Communicates strictly with the campus portal gateway (`10.254.0.254`).
* **Open Source & Zero Telemetry**: No analytics, no tracking, and fully auditable code.

---

## 💻 Build from Source

```bash
npm install
npm run dev      # Local dev mode
npm run build    # Build release packages
```

MIT License
