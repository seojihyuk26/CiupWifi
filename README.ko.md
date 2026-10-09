# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi 아이콘" />
</p>

<p align="center">
  <b>파리 국제대학촌 (CIUP) "WifiCity" 와이파이 자동 로그인 & 끊김 방지 프로그램</b><br>
  <i>매번 브라우저 열고 아이디/비밀번호 입력할 필요 없이 항상 연결을 유지해 줍니다.</i>
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
> **중요 안내 (비공식 소프트웨어)**: 본 프로그램은 기숙사 거주 학생이 개인적으로 제작한 오픈소스 편의 도구입니다. 파리 국제대학촌 (CIUP) 본부 행정처 및 IT 전산 부서와 **어떠한 공식적인 제휴나 인가 관계도 없는 비공식 앱**입니다.

---

## 🤔 왜 이 앱이 필요한가요? (제작 목적)

파리 국제대학촌(CIUP) 기숙사 와이파이(**WifiCity**)를 사용해 보셨다면 누구나 겪는 불편함이 있습니다:
- 몇 시간마다 인터넷 세션이 만료되어 갑자기 연결이 끊깁니다.
- 그때마다 브라우저를 띄우고, 로딩을 기다려 아이디와 비밀번호를 매번 다시 쳐야 합니다.
- 과제, 화상 통화, 유튜브 시청 중에 예고도 없이 인터넷이 뚝 끊기기 일쑤입니다.

**CiupWifi는 이 문제를 완전히 해결해 줍니다.**  
앱을 설치해 두면 컴퓨터나 휴대폰 백그라운드에서 조용히 대기하며, 와이파이에 연결될 때 자동으로 로그인해 줍니다. 또한 평소 연결이 끊기는 주기를 스스로 학습하여, **인터넷이 끊기기 몇 분 전에 미리 세션을 연장**해 줌으로써 작업이 중단되지 않도록 보호합니다.

---

## 🚀 사용법 (초간단 3단계)

최초에 **딱 한 번**만 설정하면 됩니다:

1. 기기에서 기숙사 와이파이 이름인 **WifiCity**에 연결합니다.
2. **CiupWifi 앱을 실행**하고, 기숙사 와이파이 **아이디(Username)**와 **비밀번호(Password)**를 입력한 뒤 **Connect**를 누릅니다.
3. **끝!** 이제 앱 창을 닫으셔도 됩니다.
   - PC/Mac: 화면을 닫아도 시계 옆 작업표시줄(트레이)에서 조용히 계속 실행됩니다.
   - 와이파이가 끊기기 직전에 앱이 알아서 백그라운드에서 자동으로 재연결을 진행합니다.

---

## 📥 다운로드

**[GitHub 최신 버전 다운로드 페이지](https://github.com/seojihyuk26/CiupWifi/releases/latest)**에서 본인의 기기에 맞는 파일을 받으세요:

| 사용하는 기기 | 다운로드 파일 | 설치 방법 |
|---|---|---|
| **Windows PC** | `CiupWifi_x.x.x_x64-setup.exe` | 다운로드 후 더블 클릭하여 설치 및 실행 |
| **Mac (맥북/아이맥)** | `CiupWifi_x.x.x_universal.dmg` | 파일을 열고 `CiupWifi`를 응용 프로그램(Applications)으로 드래그<br>*(보안 경고 발생 시: 아이콘 우클릭 > **열기** 선택)* |
| **Android 스마트폰** | `app-universal-release-unsigned.apk` | 스마트폰에서 APK 파일을 다운로드 후 바로 설치 |
| **아이폰 / 아이패드** | [`wifiLogin.js`](wifiLogin.js) (브라우저 스크립트) | [Orion Browser](https://kagi.com/orion/) 또는 Safari의 [Userscripts 확장 프로그램](https://apps.apple.com/app/userscripts/id1463298887) 설치 후 `wifiLogin.js` 스크립트를 추가하여 사용 |

---

## 🔒 내 비밀번호는 안전한가요?

**네, 100% 안전합니다.**
- 입력하신 계정 정보는 **본인의 기기 로컬 저장소에만** 안전하게 저장됩니다.
- 외부 서버나 개발자에게 어떠한 정보도 절대 전송되지 않습니다.
- 오직 기숙사 내부 로그인 주소(`10.254.0.254`)와만 직접 통신합니다.
- 모든 소스 코드가 투명하게 공개된 오픈소스 소프트웨어입니다.

---

## 🙏 크레딧 및 참조 (감사의 글)

본 프로젝트는 CIUP 거주 학생 커뮤니티의 앞선 연구와 오픈소스 프로젝트에 많은 빚을 지고 있습니다:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: CIUP FortiGate 시스템의 `1000`번 포트 및 `magic` 보안 인증 토큰 구조를 규명한 훌륭한 Windows/Linux 자동화 스크립트 프로젝트입니다.
- **[Ranadeep Biswas의 Gist](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: 리눅스 환경에서 WifiCity 자동 로그인을 구현했던 원작 쉘 스크립트입니다.

CiupWifi는 이러한 기술적 원리를 바탕으로, 비개발자 거주 학생 누구나 편하게 사용할 수 있는 현대적인 그래픽(GUI) 앱으로 완성되었습니다.

---

## 📄 라이선스

본 프로그램은 [MIT 라이선스](LICENSE)에 따라 자유롭게 배포됩니다.
