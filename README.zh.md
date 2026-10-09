# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 图标" />
</p>

<p align="center">
  <b>巴黎国际大学城 (CIUP) "WifiCity" 校园无线网络自动登录与重连工具</b><br>
  <i>无需反复输入账号密码，网络断开时自动静默重连。</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md"><b>中文</b></a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **非官方说明**：本软件是由大学城住宿学生独立开发的开源工具，与 CIUP 校方管理处或 IT 部门无官方关联。

---

## 📥 下载安装（最新版本）

请从 **[GitHub Releases 页面](https://github.com/seojihyuk26/CiupWifi/releases/latest)** 下载对应系统的安装包：

| 平台 | 安装文件 | 说明 |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | 标准安装包（若弹出 SmartScreen 保护：点击 **更多信息** → **仍要运行**） |
| **macOS** | `CiupWifi_*_universal.dmg` | 拖入 `Applications`（若提示已阻止：**系统设置** → **隐私与安全性** → **仍要打开**） |
| **Android** | `CiupWifi.apk` | 安卓手机及平板专用 APK 安装包 |
| **iOS / iPadOS / Mac** | [**Greasy Fork 脚本**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Safari（[Userscripts 扩展](https://apps.apple.com/app/userscripts/id1463298887)）或 Orion 浏览器 1 秒快速安装 |

---

## 🚀 使用方法（30秒完成）

1. 将设备连接至宿舍 Wi-Fi **WifiCity**。
2. 打开 **CiupWifi**，输入校园网 **账号与密码**，点击 **Connect**。
3. **连接成功后可直接关闭窗口（`[X]`）。**
   - 自动在操作系统任务计划程序（Windows/Mac）中注册后台无感知重连任务。
   - 仅在网络掉线时后台 0.3 秒静默重连，**日常空闲内存占用接近 0 MB**。

---

## 🛡️ 首次运行安全提示（Windows / macOS / Android）

作为无企业付费数字签名的独立开源软件，各系统在首次启动时可能会弹出拦截提示。通过官方步骤即可安全运行：

* **🪟 Windows (SmartScreen)**：
  - 弹出 *"Windows 已保护你的电脑"* 时：点击 **[更多信息]** → **[仍要运行]**。
* **🍏 macOS (Gatekeeper)**：
  - 若提示已阻止：点击 **[取消]** → 打开 Mac **[系统设置]** → **[隐私与安全性]** → 在安全性区域点击 **[仍要打开]** → 验证密码或指纹。
* **🤖 Android (Google Play 保护机制)**：
  - 若拦截安装：点击 **[了解详情]** (∨) → 点击 **[仍要安装]**（或在浏览器中开启 *允许安装未知应用*）。
* **🌐 免安装方案（浏览器脚本）**：
  - 若不想在系统中配置安全例外，可直接在 Safari、Chrome、Firefox 中使用我们的 [**Greasy Fork 脚本**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1)，100% 在浏览器沙箱内无权限运行。

---

## 🔒 隐私与安全性

* **100% 本地保存**：账号信息仅保存在本设备本地，绝不上发至任何外部服务器。
* **仅直连本地网关**：仅与宿舍内部认证网关（`10.254.0.254`）直接通信。
* **完全开源无追踪**：无任何数据收集与遥测代码，代码全公开透明。

---

## 💻 源码构建

```bash
npm install
npm run dev      # 开发调试
npm run build    # 打包构建
```

MIT 许可证
