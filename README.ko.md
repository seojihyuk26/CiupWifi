# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 아이콘" />
</p>

<p align="center">
  <b>파리 국제대학촌 (CIUP) "WifiCity" 와이파이 자동 로그인 및 세션 유지 유틸리티</b>
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
> **안내**: 본 프로그램은 거주 학생이 개발한 독립적인 오픈소스 유틸리티입니다. 파리 국제대학촌 (CIUP) 본부 및 IT 부서와 **공식 제휴나 인가 관계가 없는 비공식 프로그램**입니다.

---

## 개요

파리 국제대학촌(CIUP)의 캠퍼스 와이파이(**WifiCity**)는 일정 시간마다 캡티브 포털 세션이 만료되어, 거주자가 브라우저를 열고 계정 정보를 다시 입력해야 하는 번거로움이 있습니다.

**CiupWifi**는 이 과정을 자동화합니다:
- 브라우저 창을 띄우지 않고 백그라운드에서 직접 인증을 수행합니다.
- 실제 세션 지속 시간을 기록하고, 연결이 끊기기 전에 미리 재인증을 수행하여 연결을 유지합니다.
- 시스템 트레이에 상주하며 최소한의 메모리(< 15 MB)만 사용합니다.

---

## 사용법

1. 기기를 기숙사 와이파이인 **WifiCity**에 연결합니다.
2. **CiupWifi**를 실행하고, 캠퍼스 계정 아이디와 비밀번호를 입력한 뒤 **Connect**를 누릅니다.
3. 연결 즉시 프로그램 창을 닫으셔도 됩니다(`[X]`). 운영체제(Windows 작업 스케줄러 / macOS LaunchAgent)에 백그라운드 재인증이 자동 등록되어 메모리 낭비 없이 끊겼을 때만 조용히 로그인됩니다.

---

## 다운로드

설치 파일은 **[GitHub 릴리즈 페이지](https://github.com/seojihyuk26/CiupWifi/releases/latest)**에서 받을 수 있습니다:

| 운영체제 | 설치 파일 | 안내 |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (또는 `.msi`) | 표준 Windows 인스톨러입니다.<br>*(PC 보호 경고 창 발생 시: **추가 정보** 클릭 > **실행**)* |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | 파일을 열고 `CiupWifi.app`을 응용 프로그램으로 드래그합니다.<br>*(보안 경고 시 아래 macOS 가이드 참조)* |
| **Android** | `CiupWifi.apk` | 안드로이드 스마트폰/태블릿용 서명된 APK 설치 파일입니다. |
| **iOS / iPadOS** | [**Greasy Fork (원클릭 설치)**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) / [`wifiLogin.js`](wifiLogin.js) | Safari의 [Userscripts 확장](https://apps.apple.com/app/userscripts/id1463298887) 또는 [Orion Browser](https://kagi.com/orion/)에서 1초 만에 설치 및 자동 업데이트가 지원됩니다. |

### 🍏 macOS 보안 경고 해결법 ("악성 코드 확인할 수 없음")

macOS는 애플 개발자 등록($99/년)을 거치지 않은 오픈소스 앱을 인터넷에서 다운로드할 경우 Gatekeeper 보안 정책에 의해 *"Apple은 사용자의 Mac을 손상시키거나 개인정보에 침입할 수 있는 악성 코드가 없다고 확인할 수 없습니다"* 경고창을 띄웁니다. 아래 두 가지 방법 중 하나로 1초 만에 해결할 수 있습니다:

#### ⚡ 방법 1: 터미널 명령어 1줄 실행 (가장 추천, 영구 해제)
터미널(Terminal)을 열고 아래 명령어를 입력한 뒤 Enter를 누릅니다:
```bash
xattr -cr /Applications/CiupWifi.app
```
> macOS가 다운로드 파일에 부여한 격리 플래그(`com.apple.quarantine`)를 즉시 제거하여, 이후부터는 일반 앱처럼 더블클릭으로 바로 열립니다.

#### ⚙️ 방법 2: 시스템 설정에서 "확인 없이 열기" (GUI)
1. 경고창에서 **[완료]** 또는 **[취소]**를 누릅니다.
2. Mac **시스템 설정 (System Settings)** → **개인정보 보호 및 보안 (Privacy & Security)**으로 이동합니다.
3. 화면 아래로 스크롤하여 **보안** 섹션에서 *"CiupWifi" 사용이 차단되었습니다* 옆의 **[확인 없이 열기] (Open Anyway)** 버튼을 클릭합니다.
4. 비밀번호 또는 Touch ID를 인증하면 정상 실행됩니다.

### 📱 iOS / iPadOS 사용 가이드 (Safari / Orion)

iOS에서는 외부 설치 파일(.ipa) 직접 배포가 제한되므로, Safari 유저스크립트 방식을 통해 동일하게 자동 로그인을 지원합니다:

1. **확장 프로그램 설치 (택1)**:
   * **Safari 브라우저**: App Store에서 무료 [Userscripts 앱](https://apps.apple.com/app/userscripts/id1463298887)을 설치한 뒤, 아이폰 `설정 > Safari > 확장 프로그램`에서 활성화합니다.
   * **Orion 브라우저**: [Orion Browser](https://kagi.com/orion/) 설치 (크롬/파이어폭스 확장 기본 지원).
2. **스크립트 원클릭 설치**:
   * [Greasy Fork 스크립트 페이지](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1)를 열고 **[Install this script]** 버튼을 누릅니다.
3. **사용**:
   * 와이파이 연결 시 뜨는 포털 창에서 평소처럼 1회만 로그인하면, 계정 정보가 브라우저 로컬 저장소에 저장되어 이후 캡티브 창이 뜰 때마다 1초 만에 자동 로그인됩니다.

---

## 개인정보 보호 및 보안

- **로컬 전용 저장**: 계정 정보는 외부로 전송되지 않으며, 사용자 기기의 로컬 앱 데이터 폴더에만 저장됩니다.
- **내부 게이트웨이 직접 통신**: 요청은 오직 기숙사 내부 로그인 게이트웨이(`10.254.0.254`)로만 전송됩니다.
- **텔레메트리 없음**: 사용자 추적, 분석 통계 수집, 외부 서버 연동이 일절 없습니다.
- **오픈소스**: 전체 코드가 투명하게 공개되어 있어 누구나 검증할 수 있습니다.

---

## 기술 상세 및 아키텍처

개발자 및 내부 동작 원리가 궁금한 사용자를 위한 내용입니다:

### 1. 직접 네이티브 HTTP 통신
브라우저나 무거운 웹뷰를 거치지 않고, Rust 기반 백엔드(`reqwest`)를 통해 경량으로 직접 통신합니다:
- `http://www.google.com/gen_204`에 프로브를 보내 캡티브 리다이렉트 발생 여부를 감지합니다.
- FortiGate 게이트웨이(`http://10.254.0.254:1000/fgtauth?...`)로의 302 리다이렉트를 감지합니다.
- 동적 세션 토큰(`magic`)을 추출하고, `ft_un`/`username`, `ft_pd`/`password`, `magic`이 모두 포함된 듀얼 호환 폼 데이터를 POST로 전송합니다.
- 후속 HTTP 204 응답을 통해 실제 인터넷 연결 상태를 검증합니다.

### 2. 선제적 재연결 타이머
- 절전 모드나 일시적 신호 불량 등 단순 노이즈를 거르기 위해 **1시간 이상 유지된 정상 세션**만 최근 최대 3개까지 기록합니다.
- 기록된 시간 중 **가장 짧았던 만료 시간(최솟값)**을 기준으로 삼아, 만료 5분 전에 미리 자동으로 연결을 갱신합니다.
- 앱이 선제적으로 갱신한 세션은 자연 만료 기록에 추가하지 않아 기준 시간이 왜곡되지 않도록 유지합니다.

---

## 참고 및 크레딧

본 프로젝트는 CIUP 학생 개발자 커뮤니티의 선행 분석에 기반하여 제작되었습니다:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: FortiGate의 `1000`번 포트 및 `magic` 토큰 추출 구조를 분석하고 초기 CLI 스크립트를 제공했습니다.
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: 리눅스용 WifiCity 자동 로그인 쉘 스크립트 원작자입니다.

---

## 소스코드 빌드

### 요구사항
- Node.js 20+
- Rust 1.77+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)

```bash
# 의존성 설치
npm install

# 개발 모드 실행
npm run dev

# 릴리즈 패키지 빌드
npm run build
```

---

## 라이선스

[MIT License](LICENSE)에 따라 배포됩니다.
