# Google Play Console Submission Guide — CiupWifi (`org.ciup.wifi`)

All technical hurdles required by Google Play (targetSdk 36 / Android 16, NDK r28, 16 KB ELF page alignment, monotonic `versionCode`, declarative `network_security_config.xml`, zero `TrustManager` bypasses, zero external APK self-updater, edge-to-edge system bar insets, privacy policy, and store graphics) have been handled in `v1.4.0`.

---

## 1. Register Signing Secrets in GitHub (One-Time, ~2 min)

A persistent upload keystore (`.play-secrets/ciupwifi-upload.jks`) has been generated locally and excluded via `.gitignore`.
Open **GitHub Repo → Settings → Secrets and variables → Actions → New repository secret** and add the 4 secrets listed in `.play-secrets/GITHUB_SECRETS_SETUP.txt`:

1. `ANDROID_KEYSTORE_BASE64`
2. `ANDROID_KEYSTORE_PASSWORD`
3. `ANDROID_KEY_ALIAS`
4. `ANDROID_KEY_PASSWORD`

*(Optional for 100% automated CI upload to Play Store Internal Track)*:
5. `PLAY_SERVICE_ACCOUNT_JSON` — Service Account JSON created in Google Cloud + granted Release Manager access in Play Console → **Users and permissions**.

> ⚠️ **Important**: Back up `.play-secrets/ciupwifi-upload.jks` somewhere safe (e.g., password manager / encrypted drive). Never lose this upload keystore or commit `.play-secrets/`.

---

## 2. Recommended Rollout Track

- **Fastest / Personal + Friends (Up to 100 testers, zero 12-tester wait)**:
  Upload `CiupWifi.aab` to **Testing → Internal testing**, add your Google email to the tester list, open the opt-in link on your Galaxy phone, and install via Play Store. Future updates auto-update through Play Store.
- **Public Production Release**:
  If using a personal developer account created after Nov 2023, Google Play requires **Closed testing (12+ opted-in testers for 14 continuous days)** before applying for Production access.

---

## 3. Store Listing Copy (Ready to Paste)

> **Policy tip**: Include the unofficial disclaimer right at the top of the Full Description so Google Play reviewers do not flag the app for **Impersonation** of CIUP.

### English (`en-US` — Default)
- **App name**: `CiupWifi - WifiCity Auto Login` *(29/30 chars)*
- **Short description** *(max 80 chars)*:
  `Unofficial auto-login utility for CIUP campus WifiCity captive portal.` *(71 chars)*
- **Full description**:
  ```
  Unofficial open-source utility developed independently by a resident for Cité Internationale Universitaire de Paris (CIUP) residents. Not affiliated with CIUP administration.

  Tired of manually typing your credentials every few hours when the WifiCity captive portal expires? CiupWifi automates campus Wi-Fi portal sign-in so your phone stays connected just like home Wi-Fi.

  KEY FEATURES
  • Event-Driven & Low Power: No polling loop. Android's native ConnectivityManager wakes the app when joining WifiCity or when portal re-authentication is needed.
  • Multi-Port Portal Detection: Automatically detects CIUP captive portal gateways across HTTP/HTTPS and dynamic ports.
  • Manual Disconnect Control: Tap "Disconnect WifiCity" to stay offline until you tap Connect or rejoin the network.
  • Privacy-First & Minimal Permissions: Requires NO Location permission, NO Notification permission, and NO background foreground-service banner.
  • 100% Local Storage: Your campus ID and password remain strictly inside Android's private app storage and are sent directly only to your campus gateway.

  OPEN SOURCE
  Inspect every line of code on GitHub:
  https://github.com/seojihyuk26/CiupWifi
  ```

### 한국어 (`ko-KR`)
- **앱 이름**: `CiupWifi - WifiCity 자동 로그인`
- **간단한 설명**: `파리 국제대학촌(CIUP) WifiCity 공용 와이파이 자동 로그인 유틸리티 (비공식)`
- **자세한 설명**:
  ```
  파리 국제대학촌(CIUP) 기숙사 거주자가 개발한 비공식 오픈소스 유틸리티입니다. CIUP 공식 기관 앱이 아닙니다.

  매번 만료되는 WifiCity 로그인 창에 아이디와 비밀번호를 다시 입력할 필요 없이, 일반 와이파이처럼 자동으로 인증하고 세션을 유지해 줍니다.

  주요 기능
  • 배터리 소모 최소화: 백그라운드 상시 실행 없이, Android OS 네트워크 이벤트가 발생할 때만 순간 동작합니다.
  • 모든 포트 자동 감지: 1000포트 외 다양한 캠퍼스 포털 포트를 자동으로 탐색합니다.
  • 연결 끊기 존중: 앱에서 "Disconnect WifiCity"를 누르면 직접 재연결하거나 와이파이를 껐다 켤 때까지 재접속하지 않습니다.
  • 민감 권한 0개: 위치 정보, 알림, 포그라운드 서비스 알림을 전혀 요구하지 않습니다.
  • 100% 기기 내부 저장: 계정 정보는 외부 서버가 아닌 사용자 스마트폰 내부에만 저장됩니다.
  ```

### Français (`fr-FR`)
- **Nom de l'application**: `CiupWifi - WifiCity Auto Login`
- **Brève description**: `Utilitaire non officiel de connexion automatique au portail WifiCity (CIUP).`
- **Description complète**:
  ```
  Application open-source non officielle développée par un résident pour la Cité Internationale Universitaire de Paris (CIUP). Sans affiliation officielle avec la CIUP.

  CiupWifi automatise l'authentification sur le portail captif « WifiCity » dès que votre téléphone se connecte au réseau ou lorsque la session expire.

  FONCTIONNALITÉS
  • Déclenchement natif basse consommation : aucune tâche en boucle, déclenché uniquement par Android lorsqu'un portail captif WifiCity est détecté.
  • Zéro permission sensible : aucune demande de localisation ni de notification.
  • Stockage 100 % local : vos identifiants restent uniquement sur votre appareil et ne sont transmis qu'au portail du campus.
  ```

---

## 4. App Content & Policy Declarations (Play Console)

| Section | Exact Answer |
|---|---|
| **Privacy policy URL** | `https://github.com/seojihyuk26/CiupWifi/blob/main/PRIVACY_POLICY.md` |
| **Ads** | **No, my app does not contain ads** |
| **App category** | `Tools` (도구) |
| **Contact email** | Your developer email |
| **External website** | `https://github.com/seojihyuk26/CiupWifi` |
| **Data safety — Collect or share user data?** | **Yes, collects; no sharing with unrelated third parties** (campus credentials are transmitted directly to the CIUP portal for authentication) |
| **Data safety — Data type** | **Personal info → User IDs**; required; app functionality / account management; no advertising or analytics |
| **Data safety — Encrypted in transit?** | **No** (the local captive portal may use HTTP; do not claim universal encryption) |
| **Data safety — Deletion requests** | **Yes** (users can overwrite credentials, clear app storage, or uninstall; no developer server copy exists) |
| **App access (Reviewer Instructions)** | Select **"All or some functionality is restricted"** and paste this reviewer note: |

### Data Safety Credential Guidance

The app sends the campus username and password off-device to the CIUP portal when the user signs in. Do not describe this as purely on-device processing in the Data safety form. Declare **Personal info → User IDs** as collected for app functionality and account management. The app does not share credentials with the developer, advertisers, analytics providers, or unrelated third parties. Because the local portal may use HTTP, answer **No** to universal encryption in transit rather than making a misleading claim.

### Cleartext Traffic Note

HTTP is supported only because captive-portal gateways and connectivity probes may use plain HTTP. HTTPS uses system trust anchors only; the app does not install user certificate authorities or bypass TLS validation. If Play Console asks for a cleartext-traffic justification, explain this captive-portal requirement.

### Reviewer Note (App Access)
```
CiupWifi is an unofficial campus Wi-Fi captive-portal login helper strictly for residents on the physical CIUP ("WifiCity") campus network in Paris, which uses a local private gateway (e.g. 10.254.0.254) inaccessible from external networks.

Because the physical campus Wi-Fi network is required to complete an end-to-end portal submission, reviewers outside the campus will see the local credential entry screen and the "Waiting for WifiCity network" / "Not on WifiCity" status cards cleanly with zero network errors or crashes. There are no demo/cloud accounts operated by the developer.
```

---

## 5. Graphic Assets Ready to Upload (`play-store/assets/`)

All required PNGs are generated in [`play-store/assets/`](assets/):
- **App icon (512 × 512)**: `icon-512.png`
- **Feature graphic (1024 × 500)**: `feature-graphic-1024x500.png`
- **Phone screenshots (1080 × 1920)**:
  - `screenshot-1-connected.png`
  - `screenshot-2-login.png`
  - `screenshot-3-privacy.png`
