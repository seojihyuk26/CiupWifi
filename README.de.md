# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Schnelle automatische Anmeldung und proaktive Sitzungserneuerung für das Wi-Fi der Cité internationale universitaire de Paris (CIUP).</b>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.de.md"><b>Deutsch</b></a>
</p>

> [!WARNING]
> **Wichtiger Hinweis**: Dies ist ein **inoffizielles Open-Source-Tool**, das unabhängig von Bewohnern entwickelt wurde. Es ist **in keiner Weise** mit der Verwaltung oder den IT-Diensten der **Cité internationale universitaire de Paris (CIUP)** verbunden, autorisiert oder von diesen unterstützt.

---

## 💡 Was ist CiupWifi?

In der Cité internationale universitaire de Paris (CIUP) beendet das Captive Portal regelmäßig aktive Sitzungen, sodass sich Bewohner mehrmals täglich manuell im Browser neu einloggen müssen.

**CiupWifi** ist eine native plattformübergreifende Anwendung (Windows, macOS, Android), entwickelt mit **Tauri v2**, die diesen gesamten Prozess automatisiert:
- **Kein Browser erforderlich**: Meldet sich über direkte native HTTP-Anfragen bei `10.254.0.254` an – ganz ohne störende Browserfenster oder WebViews.
- **Proaktive Wiederverbindung**: Erfasst die typische Sitzungsdauer und erneuert die Verbindung automatisch einige Minuten *vor* dem Ablauf der Sitzung.
- **Ressourcenschonend & akkufreundlich**: Läuft unauffällig im Infobereich (System Tray) mit minimaler Speichernutzung (< 15 MB).
- **Datenschutz**: Ihre Anmeldedaten werden ausschließlich lokal auf Ihrem Gerät gespeichert.

---

## 📥 Download & Installation

Besuchen Sie die **[neueste GitHub Releases-Seite](https://github.com/seojihyuk26/CiupWifi/releases/latest)**, um das passende Paket herunterzuladen:

| Plattform | Datei | Anleitung |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (oder `.msi`) | Installationsdatei ausführen und über das Startmenü öffnen. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | `.dmg` öffnen und `CiupWifi.app` in den Ordner Programme ziehen.<br>*(Falls macOS vor einem nicht identifizierten Entwickler warnt: Rechtsklick > **Öffnen**)* |
| **Android** | `app-universal-release-unsigned.apk` | APK herunterladen und installieren (Installation aus unbekannten Quellen bei Aufforderung erlauben). |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Auf dem iPhone [Orion Browser](https://kagi.com/orion/) oder die Safari-Erweiterung [Userscripts](https://apps.apple.com/app/userscripts/id1463298887) verwenden und `wifiLogin.js` installieren. |

---

## 🚀 Verwendung

1. Mit dem Wi-Fi-Netzwerk der CIUP **verbinden**.
2. **CiupWifi starten**.
3. Einmalig **Benutzername** und **Passwort** eingeben und auf **Connect** klicken.
4. Fertig!
   - Auf dem PC: Das Fenster kann geschlossen werden; das Programm läuft im Hintergrund im System Tray weiter.
   - CiupWifi erneuert die Verbindung automatisch vor Ablauf der Sitzung.

---

## 🔒 Sicherheit & Datenschutz

- **Keine externen Server**: Keine Telemetrie oder Datenübertragung an Dritte.
- **Lokale Speicherung**: Zugangsdaten werden sicher im lokalen Anwendungsdatenverzeichnis gespeichert.
- **Vollständig Open Source**: Der Quellcode ist öffentlich einsehbar.

---

## 📄 Lizenz & Rechtlicher Hinweis

Veröffentlicht unter der [MIT-Lizenz](LICENSE).  
**Inoffizielle Anwendung**: CiupWifi ist ein unabhängiges Gemeinschaftsprojekt und steht in keiner offiziellen Verbindung zur Cité internationale universitaire de Paris (CIUP).

