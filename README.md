# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Automatic login tool for the "WifiCity" Wi-Fi network at Cité internationale universitaire de Paris (CIUP).</b><br>
  <i>Stay connected without constantly typing your username and password.</i>
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
> **Important Note (Unofficial Tool)**: This is an independent, student-made helper app. It is **NOT** an official app and is not affiliated with the administration or IT department of the Cité internationale universitaire de Paris (CIUP).

---

## 🤔 Why do I need this? (Purpose)

If you live at CIUP and use the campus Wi-Fi (**WifiCity**), you already know the frustration:
- Every few hours, the network disconnects you.
- You have to open a browser, wait for the login page to load, and type your username and password again.
- While video calling, studying, or streaming, your connection suddenly cuts out without warning.

**CiupWifi fixes this completely.**  
Once installed, it runs quietly in the background on your computer or phone. It logs you into the Wi-Fi automatically and learns when your connection usually expires, renewing it *before* you get disconnected.

---

## 🚀 How to Use (Simple 3 Steps)

You only need to set it up **once**:

1. **Connect** your device to the campus Wi-Fi named **WifiCity**.
2. **Open CiupWifi**, type your campus Wi-Fi **Username** and **Password**, and click **Connect**.
3. **Done!** You can close the app window.
   - It will stay running quietly in the taskbar/tray (on PC/Mac) or background.
   - Whenever your connection is about to expire, it automatically reconnects for you.

---

## 📥 Downloads

Click to download for your device from our **[Latest Release Page](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Your Device | File to Download | Simple Setup |
|---|---|---|
| **Windows PC** | `CiupWifi_x.x.x_x64-setup.exe` | Download, double-click to install, and open it. |
| **Mac (Apple)** | `CiupWifi_x.x.x_universal.dmg` | Open the file, drag `CiupWifi` into your Applications folder.<br>*(If Mac shows a security warning: right-click the app > click **Open**)* |
| **Android Phone** | `app-universal-release-unsigned.apk` | Download and install the APK file on your phone. |
| **iPhone / iPad** | [`wifiLogin.js`](wifiLogin.js) (Browser Script) | Install [Orion Browser](https://kagi.com/orion/) or the [Userscripts extension for Safari](https://apps.apple.com/app/userscripts/id1463298887), then add the `wifiLogin.js` script. |

---

## 🔒 Is my password safe?

**Yes, 100%.**
- Your username and password are stored **only on your own device**.
- No information is ever sent to any third-party server or developer.
- The app only talks directly to the campus Wi-Fi login page (`10.254.0.254`).
- The entire project is free, open-source, and transparent.

---

## 🙏 Credits & Acknowledgments

This application was inspired by and built upon previous community research on the CIUP captive portal:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: Excellent command-line automation scripts for Windows and Linux that uncovered the FortiGate `magic` authentication tokens and port `1000`.
- **[Ranadeep Biswas's Gist](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: The original Linux shell script for automated WifiCity login.

CiupWifi packages these technical breakthroughs into a user-friendly, modern graphical app for everyday residents.

---

## 📄 License

Distributed under the [MIT License](LICENSE).
