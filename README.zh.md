# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 图标" />
</p>

<p align="center">
  <b>巴黎国际大学城 (CIUP) “WifiCity” 校园无线网络自动登录与会话保持工具。</b>
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
> **免责声明**：本软件为住户学生自主开发的独立开源辅助工具。**绝非**巴黎国际大学城 (CIUP) 官方应用，且与大学城管理层或网络信息部门无任何隶属关系。

---

## 概述

巴黎国际大学城 (CIUP) 的校园 Wi-Fi 网络 (**WifiCity**) 设有定期的强制门户超时机制，导致住户每天必须多次重新在浏览器中输入账号密码进行身份验证。

**CiupWifi** 旨在自动化此流程：
- 后台静默完成认证，无需弹出任何浏览器窗口。
- 跟踪实际网络会话有效时长，并在断线前主动提前续期。
- 最小化常驻系统托盘，内存占用极低（< 15 MB）。

---

## 使用方法

1. 将设备连接至大学城 **WifiCity** 无线网络。
2. 打开 **CiupWifi**，输入您的大学城账号与密码，点击 **Connect**。
3. 关闭窗口即可。程序会常驻任务栏托盘，并在后台自动处理后续的所有登录与超时续期。

---

## 下载安装

安装包可在 **[GitHub Releases 页面](https://github.com/seojihyuk26/CiupWifi/releases/latest)** 获取：

| 操作系统 | 安装包文件 | 说明 |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (或 `.msi`) | 标准 Windows 安装包，可在开始菜单启动。 |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | 将 `CiupWifi.app` 拖入 Applications 目录。<br>*(若出现安全提示：右键点击图标 > 选择 **打开**)* |
| **Android** | `app-universal-release-unsigned.apk` | 适用于安卓设备的独立 APK 安装包。 |
| **iOS / iPadOS** | [`wifiLogin.js`](wifiLogin.js) (脚本) | 可在 iOS 上通过 [Orion Browser](https://kagi.com/orion/) 或 Safari [Userscripts 扩展](https://apps.apple.com/app/userscripts/id1463298887) 运行 `wifiLogin.js`。 |

---

## 隐私与安全

- **仅限本地存储**：登录凭据仅保存在本机操作系统的应用数据目录中。
- **直接网关通信**：请求仅直接发送至大学城内部网关地址 (`10.254.0.254`)。
- **无数据回传**：不包含任何数据埋点、统计或第三方服务器交互。
- **全开源透明**：全部源代码均公开以供安全审查。

---

## 技术实现与架构

供开发者及感兴趣的用户参考的技术细节：

### 1. 原生直接 HTTP 认证
与在 WebView 或浏览器中注入脚本不同，CiupWifi 采用轻量 Rust 后端（基于 `reqwest`）：
- 探测 `http://www.google.com/gen_204` 以识别强制网络跳转。
- 拦截重定向至 FortiGate 网关接口（`http://10.254.0.254:1000/fgtauth?...`）。
- 提取动态会话认证凭证（`magic`），并提交双重兼容的表单参数（`ft_un`/`username`, `ft_pd`/`password`, `magic`）。
- 通过后续的 HTTP 204 请求验证连通性是否真正恢复。

### 2. 预测性主动续期算法
- 维护自然会话持续时长的滚动历史记录（最多 30 条）。
- 计算统计学上的最小有效会话周期 ($T_{\text{min}}$)。
- 在 $T_{\text{min}} - T_{\text{已过去时间}} - 5\text{ 分钟}$ 触发自动重新认证，防止在重要工作或观看视频时意外断线。
- 主动发起的重连会被标记并排除在自然断线统计之外，以确保预测模型精度。

---

## 致谢与参考项目

本项目深受 CIUP 学生社区先期技术探索的启发：
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**：发现了 FortiGate 系统的 `1000` 端口以及 `magic` 认证参数，并提供了实用的 Windows/Linux 命令行脚本。
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**：最初用于 Linux 系统的 WifiCity 登录脚本原型。

---

## 开源许可证

遵循 [MIT 许可证](LICENSE) 发布。
