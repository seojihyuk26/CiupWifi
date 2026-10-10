# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 아이콘" />
</p>

<p align="center">
  <b>파리 국제대학촌(CIUP) "WifiCity" 와이파이 자동 로그인 & 재인증 유지 유틸리티</b><br>
  <i>인터넷이 끊겨도 로그인 창을 띄울 필요 없이 조용히 연결을 유지합니다.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md"><b>한국어</b></a>
</p>

> [!WARNING]
> **비공식 안내**: 본 프로그램은 거주 학생이 개발한 독립 오픈소스 프로그램이며, CIUP 본부 및 IT 부서와 제휴 관계가 없습니다.

---

## 📥 다운로드 (최신 버전)

최신 설치 파일은 **[GitHub 릴리즈](https://github.com/seojihyuk26/CiupWifi/releases/latest)**에서 바로 받을 수 있습니다:

| 플랫폼 | 설치 파일 | 설치 & 실행 안내 |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | 설치 후 실행 (SmartScreen 창 뜨면: **추가 정보** → **실행**) |
| **macOS** | `CiupWifi_*_universal.dmg` | `Applications`로 드래그 (차단 시: **시스템 설정** → **보안** → **확인 없이 열기**) |
| **Android** | `CiupWifi.apk` | 스마트폰/태블릿용 단일 APK 설치 |
| **iOS / iPadOS / Mac** | [**Greasy Fork 스크립트**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Safari([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) 또는 Orion 브라우저에서 1초 만에 설치 |

---

## 🚀 사용법 (30초 완료)

1. 기기를 기숙사 와이파이인 **WifiCity**에 연결합니다.
2. **CiupWifi**를 실행하고, 캠퍼스 **아이디/비밀번호**를 입력한 뒤 **Connect**를 누릅니다.
3. **창을 바로 닫으셔도 됩니다 (`[X]`).**
   - 백그라운드 재인증이 OS 작업 스케줄러(Windows/Mac)에 자동 등록됩니다.
   - 트레이에 상주하지 않고 백그라운드에서 0.3초 만에 연결을 점검·유지하므로 **메모리 소모가 0에 가깝습니다**.

---

## 🛡️ 첫 실행 보안 알림 안내 (Windows / macOS / Android)

독립 오픈소스 소프트웨어 특성상 기업용 유료 서명 인증서가 없어 각 OS에서 첫 실행 시 경고가 뜰 수 있습니다. 각 플랫폼의 공식 절차를 통해 1초 만에 안전하게 실행하실 수 있습니다:

* **🪟 Windows (SmartScreen)**:
  - *"Windows의 PC 보호"* 창 발생 시: **[추가 정보]** 클릭 → **[실행]** 클릭
* **🍏 macOS (Gatekeeper)**:
  - 차단 창에서 **[완료]** 클릭 → Mac **[시스템 설정]** → **[개인정보 보호 및 보안]** → 화면 아래 **[확인 없이 열기]** 클릭 후 인증
* **🤖 Android (Play 프로텍트)**:
  - 차단 팝업 발생 시: **[세부정보 더보기]** (∨) 클릭 → **[무시하고 설치]** 클릭 (또는 브라우저 설정에서 *출처를 알 수 없는 앱 설치* 허용)
* **🌐 무설치 대안 (브라우저 유저스크립트)**:
  - 앱 설치나 보안 설정이 번거로우신 경우 Safari, Chrome, Firefox에서 [**Greasy Fork 스크립트**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1)를 등록하시면 100% 브라우저 샌드박스 안에서 안전하게 동작합니다.

---

## 🔒 개인정보 및 보안

* **100% 로컬 저장**: 계정 정보는 외부 서버로 전송되지 않으며, 사용자 기기 로컬에만 안전하게 저장됩니다.
* **게이트웨이 직접 통신**: 캠퍼스 내부 인증 게이트웨이(`10.254.0.254`)하고만 직접 통신합니다.
* **오픈소스 & 무추적**: 텔레메트리나 데이터 수집이 일절 없으며 코드가 완전히 투명하게 공개되어 있습니다.

---

## 💻 빌드 및 개발

```bash
npm install
npm run dev      # 개발 모드
npm run build    # 배포용 바이너리 빌드
```

MIT License
