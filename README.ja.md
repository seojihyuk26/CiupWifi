# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi アイコン" />
</p>

<p align="center">
  <b>パリ国際大学都市 (CIUP) 「WifiCity」Wi-Fi自動ログイン・接続維持ツール</b><br>
  <i>毎回アカウント情報を入力する手間なく、切断時も静かに自動再認証します。</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md"><b>日本語</b></a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **非公式アプリ**：本ツールは大学都市に滞在する学生が個人で開発したオープンソースツールであり、CIUP事務局やIT部門とは無関係です。

---

## 📥 ダウンロード（最新版）

**[GitHub Releases ページ](https://github.com/seojihyuk26/CiupWifi/releases/latest)** よりダウンロードできます：

| プラットフォーム | ファイル | 説明 |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | 標準インストーラー（SmartScreenが出た場合：**詳細情報** → **実行**） |
| **macOS** | `CiupWifi_*_universal.dmg` | `Applications`にドラッグ（ブロック時：**システム設定** → **セキュリティ** → **このまま開く**） |
| **Android** | `CiupWifi.apk` | スマートフォン・タブレット用署名済みAPK |
| **iOS / iPadOS / Mac** | [**Greasy Fork スクリプト**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Safari（[Userscripts](https://apps.apple.com/app/userscripts/id1463298887)）やOrionで1クリック導入 |

---

## 🚀 使い方（30秒で完了）

1. 寮のWi-Fi **WifiCity** に接続します。
2. **CiupWifi** を起動し、キャンパスの **ID・パスワード** を入力して **Connect** をクリックします。
3. **ウィンドウはすぐに閉じて構いません（`[X]`）。**
   - OSのタスクスケジューラ（Windows / macOS）に自動登録されます。
   - 切断時のみ約0.3秒で静かに再接続するため、**常駐メモリ消費はほぼ 0 MB** です。

---

## 🛡️ 初回起動時のセキュリティ警告について（Windows / macOS / Android）

企業の有料署名証明書を持たない独立オープンソースアプリのため、各OSの初回起動時に警告が表示される場合があります。各OSの公式手順で安全に1秒で実行できます：

* **🪟 Windows (SmartScreen)**：
  - *"Windows によって PC が保護されました"* が出た場合：**[詳細情報]** をクリック → **[実行]** をクリック
* **🍏 macOS (Gatekeeper)**：
  - ブロックされた場合：**[キャンセル]** をクリック → Macの **[システム設定]** → **[プライバシーとセキュリティ]** を開き、セキュリティ項目で **[このまま開く]** をクリックして認証
* **🤖 Android (Google Play プロテクト)**：
  - インストールがブロックされた場合：**[詳細]** (∨) をタップ → **[詳細を無視してインストール]** をタップ（またはブラウザの *不明なアプリのインストール* を許可）
* **🌐 インストール不要の代替手段（ブラウザ拡張機能）**：
  - システム設定を変更したくない場合は、SafariやChromeで [**Greasy Fork スクリプト**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) をお使いください。100% ブラウザのサンドボックス内で安全に動作します。

---

## 🔒 プライバシーとセキュリティ

* **100% ローカル保存**：アカウント情報は外部に送信されず、お使いの端末内にのみ保存されます。
* **ローカルゲートウェイのみ直接通信**：認証は寮内のゲートウェイ（`10.254.0.254`）とのみ通信します。
* **完全オープンソース**：データ収集やトラッキングは一切ありません。

---

## 💻 ビルド方法

```bash
npm install
npm run dev      # 開発モード
npm run build    # バイナリビルド
```

MIT License
