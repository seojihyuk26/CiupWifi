# Privacy Policy for CiupWifi

**Effective Date:** 2026-11-28  
**App Name:** CiupWifi (`org.ciup.wifi`)  
**Developer:** CiupWifi Open Source Maintainers  
**Source Repository:** https://github.com/seojihyuk26/CiupWifi

> **Unofficial Notice:** CiupWifi is an independent, open-source community utility built for residents of the Cité Internationale Universitaire de Paris (CIUP). It is **not** an official application of CIUP or its IT department.

---

## 1. Summary

**CiupWifi does not operate developer servers, collect analytics, sell personal data, or share personal data with advertisers. Your campus credentials are transmitted only to the CIUP portal when authentication is required.**

- Zero analytics or crash telemetry
- Zero advertising SDKs
- Zero developer-operated backend servers
- Zero location, contacts, camera, microphone, or notification permissions
- Campus credentials leave your device only for direct authentication with the CIUP portal

---

## 2. How Credentials Are Stored & Used

To automate authentication on the CIUP campus **"WifiCity"** captive portal:

1. Your campus username and password are entered by you and stored **strictly locally on your device** (using Android `SharedPreferences` / local app storage inside your device's private sandbox).
2. Credentials are transmitted **only** directly from your device to the local campus captive portal gateway detected on your Wi-Fi network (such as `10.254.0.254`) when authenticating your Wi-Fi session.
3. Credentials are never sent to the app developer, advertisers, analytics providers, or any unrelated third party.
4. The portal may use HTTP on the local campus network. Where the portal offers HTTPS, the app uses the Android system trust store and does not bypass certificate validation.

---

## 3. Google Play Data Safety Disclosure

For Google Play Data safety purposes, transmitting the campus account ID off-device to the CIUP portal is treated as collection for app functionality. This credential is required to provide the app's login automation. Credentials are not shared with the developer or unrelated third parties. Because a local portal can use HTTP, the app does not claim that all credential transmissions are encrypted in transit.

## 4. Permissions Used (Android)

CiupWifi requests only the minimum system network permissions required to detect Wi-Fi connectivity changes and submit the captive portal login form:

- `INTERNET` — communicate directly with the campus captive portal gateway and verify internet connectivity
- `ACCESS_NETWORK_STATE` — detect when Android connects to a captive-portal Wi-Fi network without polling
- `CHANGE_NETWORK_STATE` — bind portal requests to the active Wi-Fi network interface and report validated connectivity to Android
- `RECEIVE_BOOT_COMPLETED` — re-register low-power OS network triggers after reboot

CiupWifi explicitly avoids requesting Location (`ACCESS_FINE_LOCATION` / `ACCESS_COARSE_LOCATION`), Background Location, or Notification permissions.

---

## 5. Data Deletion

Because all data is stored strictly on your device:
- You can overwrite your credentials anytime inside the app (**Change Account**).
- Uninstalling CiupWifi or clearing the app's storage permanently removes all stored data from your device.

---

## 6. Contact

If you have questions about privacy or security, please open an issue on the public GitHub repository:  
https://github.com/seojihyuk26/CiupWifi/issues
