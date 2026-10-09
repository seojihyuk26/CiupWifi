Ce script vous permet de vous connecter automatiquement au Wi-Fi de la Cité internationale universitaire de Paris (CIUP - WifiCity).

---

### 💻 Windows, macOS, Android 사용자라면?
브라우저 확장 프로그램 대신 전용 독립 앱을 사용하실 수 있습니다 (시스템 트레이 상주 없음, OS 자동 스케줄 등록 지원):
👉 **[CiupWifi 공식 다운로드 (GitHub)](https://github.com/seojihyuk26/CiupWifi/releases/latest)**
* **Windows** (`.exe` / `.msi`)
* **macOS** (`.dmg`)
* **Android** (`.apk`)

---

### 📱 iOS / iPadOS (iPhone, iPad) 설치 및 사용 가이드
iOS에서는 시스템 정책상 독립 앱 대신 Safari 확장 프로그램을 통해 이 스크립트를 사용합니다:

1. **확장 프로그램 준비 (택1)**:
   * **Safari 브라우저**: App Store에서 무료 [Userscripts for Safari](https://apps.apple.com/app/userscripts/id1463298887) 앱을 설치하고, 아이폰 **설정 > Safari > 확장 프로그램**에서 활성화합니다.
   * **Orion 브라우저**: [Orion Browser](https://kagi.com/orion/) 설치 (웹 익스텐션 기본 지원).
2. **스크립트 원클릭 설치**:
   * 상단의 초록색 **[Installer ce script / Install this script]** 버튼을 누르면 1초 만에 등록됩니다.
3. **최초 1회 로그인**:
   * 기숙사 와이파이에 연결하고 뜨는 포털 창에서 1회만 정상 로그인하면 계정 정보가 안전하게 브라우저 로컬 저장소에 저장되며, 이후 포털이 뜰 때마다 1초 만에 자동 로그인됩니다.

---

### 🔒 보안 및 개인정보 보호
* 모든 계정 정보는 외부 서버로 전송되지 않으며, 사용자 브라우저의 `GM_setValue` 로컬 스토리지에만 저장됩니다.
* 요청은 오직 기숙사 내부 로그인 게이트웨이(`10.254.0.254`)로만 전송됩니다.
* 소스코드 및 GitHub 저장소: [https://github.com/seojihyuk26/CiupWifi](https://github.com/seojihyuk26/CiupWifi)

