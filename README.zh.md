# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 图标" />
</p>

<p align="center">
  <b>巴黎国际大学城 (CIUP) “WifiCity” 校园 Wi-Fi 自动登录与防断线工具。</b><br>
  <i>再也不用每天反复在网页里手动输入账号密码了。</i>
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
> **重要声明（非官方辅助软件）**：本软件为住户学生个人开发的开源便利工具。本工具**绝非**巴黎国际大学城 (CIUP) 官方应用，与大学城行政或网络 IT 部门无任何关联。

---

## 🤔 为什么需要这个软件？（设计目的）

只要你住在 CIUP 并连接过大学城的无线网络（**WifiCity**），你一定遇到过这些烦恼：
- 每隔几小时网络就会自动失效，突然断网。
- 你必须手动打开浏览器，等待认证页面慢吞吞加载，然后重新敲一遍用户名和密码。
- 上课、查资料、看视频或视频通话时，网络经常毫无征兆地中断。

**CiupWifi 就是为了彻底解决这个问题而生的。**  
安装之后，它会在你的电脑或手机后台静默运行。它不仅能自动为你登录 Wi-Fi，还会根据平时断线的规律，在网络即将失效的前几分钟*主动*提前为你续期，保证你的网络畅通不掉线。

---

## 🚀 怎么使用？（简单 3 步）

你只需要设置**一次**即可：

1. 设备连上大学城的无线网络 **WifiCity**。
2. **打开 CiupWifi 软件**，输入你在大学城分配到的 **用户名 (Username)** 和 **密码 (Password)**，点击 **Connect**。
3. **完成！** 直接关闭软件窗口即可。
   - 电脑端：软件会自动最小化到任务栏右下角托盘图标，默默守护网络。
   - 以后每次连接 Wi-Fi 或快要断网时，它都会在后台自动重新认证，完全不需要你操心。

---

## 📥 下载安装

请在 **[GitHub 最新版本发布页](https://github.com/seojihyuk26/CiupWifi/releases/latest)** 下载适合你设备的版本：

| 你的设备 | 下载文件名 | 快速安装指南 |
|---|---|---|
| **Windows 电脑** | `CiupWifi_x.x.x_x64-setup.exe` | 下载后双击安装，并在开始菜单运行。 |
| **苹果电脑 (Mac)** | `CiupWifi_x.x.x_universal.dmg` | 打开文件，把 `CiupWifi` 拖进 Applications 文件夹。<br>*(若提示无法打开未识别开发者：鼠标右键点击图标 > 点击 **打开**)* |
| **安卓手机 (Android)** | `app-universal-release-unsigned.apk` | 下载 APK 文件并在手机上直接点击安装。 |
| **iPhone / iPad** | [`wifiLogin.js`](wifiLogin.js) (浏览器脚本) | 安装 [Orion Browser](https://kagi.com/orion/) 浏览器或 Safari 的 [Userscripts 插件](https://apps.apple.com/app/userscripts/id1463298887)，然后添加 `wifiLogin.js` 脚本即可。 |

---

## 🔒 我的密码安全吗？

**绝对安全。**
- 你的账号密码**仅保存在你自己设备本地**的系统安全存储中。
- 绝不向任何第三方服务器或软件作者发送任何个人数据。
- 软件只与大学城内部的认证网关页面（`10.254.0.254`）进行直接通信。
- 整个项目完全开源透明，任何人都可以随时审查代码。

---

## 🙏 致谢与参考项目

本项目深受 CIUP 学生社区此前研究成果的启发与帮助：
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**：编写了出色的 Windows 与 Linux 脚本，发现了大学城 FortiGate 系统的 `1000` 端口与 `magic` 安全认证机制。
- **[Ranadeep Biswas's Gist](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**：最初用于 Linux 系统的 WifiCity 自动登录脚本原型。

CiupWifi 将这些技术成果转化为对全体住户简单好用、无需技术背景的现代化图形软件。

---

## 📄 许可证

本项目基于 [MIT 许可证](LICENSE) 开源发布。
