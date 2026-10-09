# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Automatischer Login & Verbindungsaufrechterhaltung für das "WifiCity"-Netzwerk der Cité Internationale Universitaire de Paris (CIUP).</b><br>
  <i>Bleiben Sie online, ohne Ihre Zugangsdaten ständig neu eingeben zu müssen.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md"><b>Deutsch</b></a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Inoffizielle App**: Unabhängiges Open-Source-Dienstprogramm, entwickelt von einem studentischen Bewohner. Es besteht keine Verbindung zur CIUP-Verwaltung oder IT-Abteilung.

---

## 📥 Downloads (Aktuelle Version)

Laden Sie die Installationspakete von der **[GitHub Releases-Seite](https://github.com/seojihyuk26/CiupWifi/releases/latest)** herunter:

| Plattform | Datei | Hinweise |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Standard-Installer. (Bei SmartScreen: **Weitere Informationen** → **Trotzdem ausführen**) |
| **macOS** | `CiupWifi_*_universal.dmg` | In `Programme` ziehen. (Falls blockiert: **Systemeinstellungen** → **Sicherheit** → **Dennoch öffnen**) |
| **Android** | `CiupWifi.apk` | Signierte APK-Datei für Android-Smartphones und -Tablets. |
| **iOS / iPadOS / Mac** | [**Greasy Fork Userscript**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | 1-Klick-Installation in Safari ([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) oder Orion. |

---

## 🚀 Schnellstart (30 Sekunden)

1. Verbinden Sie Ihr Gerät mit dem Campus-WLAN **WifiCity**.
2. Starten Sie **CiupWifi**, geben Sie **Benutzername und Passwort** ein und klicken Sie auf **Connect**.
3. **Sie können das Fenster sofort schließen (`[X]`).**
   - Die Hintergrund-Reauthentifizierung wird automatisch im Aufgabenplaner Ihres Betriebssystems eingerichtet.
   - Läuft bei Verbindungsabbruch in ca. 0,3 s im Hintergrund ab – **nahezu 0 MB RAM im Leerlauf**.

---

## 🛡️ Sicherheitshinweise beim ersten Start (Windows / macOS / Android)

Als unabhängiges Open-Source-Dienstprogramm ohne kostenpflichtige Unternehmenszertifikate können Betriebssysteme beim ersten Start eine Warnung anzeigen. So fahren Sie sicher und vorschriftsmäßig fort:

* **🪟 Windows (SmartScreen)**:
  - Wenn *"Der PC wurde durch Windows geschützt"* erscheint: auf **[Weitere Informationen]** → **[Trotzdem ausführen]** klicken.
* **🍏 macOS (Gatekeeper)**:
  - Bei Blockierung: auf **[Abbrechen]** klicken → **[Systemeinstellungen]** → **[Datenschutz & Sicherheit]** öffnen → unter Sicherheit neben *"CiupWifi wurde blockiert"* auf **[Dennoch öffnen]** klicken → authentifizieren.
* **🤖 Android (Google Play Protect)**:
  - Bei Installationsblockade: auf **[Details]** (∨) → auf **[Trotzdem installieren]** tippen (oder *Installation unbekannter Apps* im Browser zulassen).
* **🌐 Installationsfreie Alternative (Browser-Userscript)**:
  - Wenn Sie keine Ausnahmen im Betriebssystem einrichten möchten, nutzen Sie unser [**Greasy Fork Userscript**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) in Safari, Chrome oder Firefox. Es läuft zu 100% isoliert in der Browser-Sandbox.

---

## 🔒 Datenschutz & Sicherheit

* **100% Lokale Speicherung**: Zugangsdaten verbleiben ausschließlich lokal auf Ihrem Gerät.
* **Nur lokales Gateway**: Kommuniziert ausschließlich mit dem internen Gateway (`10.254.0.254`).
* **Open Source**: Keine Telemetrie, keine Datensammlung.

---

## 💻 Kompilierung

```bash
npm install
npm run dev      # Entwicklungsmodus
npm run build    # Pakete erstellen
```

MIT-Lizenz
