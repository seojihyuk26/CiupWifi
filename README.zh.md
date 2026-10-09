# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 图标" />
</p>

<p align="center">
  <b>巴黎国际大学城 (CIUP) Wi-Fi 快速自动登录与预测性主动重连工具。</b>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.zh.md"><b>中文</b></a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.de.md">Deutsch</a>
</p>

> [!WARNING]
> **重要免责声明**：本软件为学生自发开发的**非官方开源工具**。本软件**绝非**由**巴黎国际大学城 (CIUP)** 官方管理层或其网络信息技术部门开发、运营、授权或拥有任何附属关系。请用户自行辨别使用。

---

## 💡 CiupWifi 是什么？

在巴黎国际大学城 (CIUP)，校园 Wi-Fi 强制门户 (Captive Portal) 经常自动断开会话，导致住户每天必须反复在网页中输入账号密码重新认证。

**CiupWifi** 是基于 **Tauri v2** 开发的原生跨平台应用（支持 Windows、macOS、Android），彻底实现认证全自动化：
- **无需打开浏览器**：通过底层原生 HTTP 请求直接向 `10.254.0.254` 提交登录认证，无弹窗扰屏，不依赖 WebView。
- **智能预测性主动重连**：自动统计您以往的会话有效时长，并在会话即将超时断开的数分钟前*主动*完成重连，保障网络不掉线。
- **轻量省电**：常驻系统托盘或后台服务，内存占用极低（< 15 MB）。
- **隐私至上**：您的登录账号与密码仅加密保存在本地设备中，绝不会上传至任何外部网络。

---

## 📥 下载与安装

请前往 **[GitHub Releases 最新版本发布页](https://github.com/seojihyuk/CiupWifi/releases/latest)** 下载适合您系统的安装包：

| 平台 | 安装文件 | 说明 |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (或 `.msi`) | 运行安装程序并在开始菜单启动。 |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | 打开 `.dmg` 镜像，将 `CiupWifi.app` 拖入 Applications。<br>*(若系统提示“未识别的开发者”，请右键点击应用并选择 **打开**)* |
| **Android** | `app-universal-release-unsigned.apk` | 下载并安装 APK。若有提示，请允许安装来自未知来源的应用。 |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (脚本) | 可在 iOS 上通过 [Orion Browser](https://kagi.com/orion/) 或 Safari [Userscripts 扩展](https://apps.apple.com/app/userscripts/id1463298887) 安装 `wifiLogin.js` 脚本使用。 |

---

## 🚀 使用指南

1. **连接**至大学城 CIUP Wi-Fi 网络。
2. **启动 CiupWifi**。
3. 首次使用时输入一次您的大学城网络 **用户名** 和 **密码**，点击 **Connect**。
4. 完成！
   - 电脑端：关闭窗口后程序会自动最小化常驻系统托盘。
   - 软件会在网络即将断开前或重新连上 Wi-Fi 时自动在后台完成重连。

---

## 🔒 安全与隐私保障

- **无远程服务器**：软件不包含任何数据埋点、统计或追踪代码。
- **本地存储**：账号密码仅存储于系统本地的标准 AppData 目录中。
- **完全开源**：全部源代码公开透明，接受任何人审查。

---

## 📄 开源许可证与法律声明

本软件遵循 [MIT 许可证](LICENSE) 开源发布。  
**非官方软件**：CiupWifi 仅为住户学生开发的第三方辅助工具，与巴黎国际大学城 (Cité internationale universitaire de Paris) 官方机构无关。
