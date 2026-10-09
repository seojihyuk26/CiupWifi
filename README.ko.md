# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 아이콘" />
</p>

<p align="center">
  <b>파리 국제대학촌 (CIUP) Wi-Fi 자동 로그인 및 세션 조기 재연결 도구</b>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md"><b>한국어</b></a> •
  <a href="README.de.md">Deutsch</a>
</p>

> [!WARNING]
> **중요 안내 (비공식 소프트웨어)**: 본 프로그램은 거주 학생이 개인적으로 제작한 **비공식 오픈소스 유틸리티**입니다. **파리 국제대학촌 (CIUP)** 본부 행정처 및 IT 전산 부서와 **어떠한 공식적인 제휴나 인가 관계도 없으며**, CIUP 공식 앱이 아닙니다.

---

## 💡 CiupWifi란?

파리 국제대학촌(Cité internationale universitaire de Paris)의 기숙사 Wi-Fi는 일정 주기마다 세션이 만료되어 매일 수시로 로그인 웹페이지를 띄우고 아이디/비밀번호를 다시 입력해야 합니다.

**CiupWifi**는 **Tauri v2** 기반으로 제작된 네이티브 크로스플랫폼 앱(Windows, macOS, Android)으로, 이 번거로운 과정을 완벽하게 자동화합니다:
- **브라우저 불필요**: 브라우저 창이나 무거운 웹뷰를 열 필요 없이, 네이티브 HTTP 요청(`10.254.0.254`)으로 즉시 백그라운드에서 직접 인증합니다.
- **예측 기반 사전 재로그인**: 과거 세션 유지 시간을 통계적으로 분석하여 세션이 끊기기 몇 분 *전*에 미리 재로그인을 수행해 인터넷 연결이 끊기지 않도록 방지합니다.
- **가벼운 메모리와 저전력**: 시스템 트레이(작업 표시줄 아이콘)에 상주하며 최소한의 리소스(< 15 MB)만 소비합니다.
- **개인정보 안전 보장**: 입력한 로그인 정보는 본인 컴퓨터의 로컬 앱 저장소에만 안전하게 저장되며, 외부 서버로 전송되지 않습니다.

---

## 📥 다운로드 및 설치

**[GitHub 최신 릴리즈 페이지](https://github.com/seojihyuk26/CiupWifi/releases/latest)**에서 본인의 기기에 맞는 설치 파일을 다운로드하세요:

| 운영체제 | 다운로드 파일 | 설치 방법 |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (또는 `.msi`) | 설치 파일 실행 후 시작 메뉴에서 앱 실행 |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | `.dmg` 열고 `CiupWifi.app`을 응용 프로그램(Applications)으로 드래그<br>*(개발자 미확인 경고 시: 마우스 우클릭 > **열기** 선택)* |
| **Android** | `app-universal-release-unsigned.apk` | APK 다운로드 후 설치 (출처를 알 수 없는 앱 설치 허용 필요) |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (유저스크립트) | iOS에서는 [Orion Browser](https://kagi.com/orion/) 또는 Safari [Userscripts 확장 프로그램](https://apps.apple.com/app/userscripts/id1463298887) 설치 후 `wifiLogin.js` 스크립트를 등록하여 사용 |

---

## 🚀 사용법

1. CIUP 기숙사 Wi-Fi에 **연결**합니다.
2. **CiupWifi를 실행**합니다.
3. 캠퍼스 Wi-Fi **아이디**와 **비밀번호**를 최초 1회 입력하고 **Connect**를 누릅니다.
4. 완료!
   - PC: 창을 닫아도 시스템 트레이로 최소화되어 백그라운드에서 자동 동작합니다.
   - 세션이 만료되기 직전에 자동으로 세션을 연장합니다.

---

## 🔒 보안 및 개인정보 보호

- **외부 서버 통신 없음**: 외부 원격 서버로 어떠한 개인정보나 로그도 전송하지 않습니다.
- **로컬 암호화 보관**: 인증 정보는 사용자의 기기 로컬 AppData 경로에만 저장됩니다.
- **100% 오픈소스**: 모든 코드는 투명하게 공개되어 있으며 직접 검증할 수 있습니다.

---

## 📄 라이선스 및 법적 고지

본 프로젝트는 [MIT 라이선스](LICENSE)에 따라 배포됩니다.  
**비공식 앱 고지**: 본 소프트웨어는 Cité internationale universitaire de Paris (CIUP)의 공식 프로그램이 아니며, 학생 커뮤니티 편의를 위해 독립적으로 제작된 오픈소스 도구입니다.

