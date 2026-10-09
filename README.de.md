# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Dienstprogramm zur automatischen Anmeldung und Sitzungsverwaltung für das Netzwerk „WifiCity“ der Cité internationale universitaire de Paris (CIUP).</b>
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
> **Hinweis**: Dies ist ein unabhängiges Open-Source-Tool, das von Bewohnern entwickelt wurde. Es ist **keine** offizielle Anwendung und steht in keiner Verbindung zur Verwaltung oder den IT-Diensten der Cité internationale universitaire de Paris (CIUP).

---

## Überblick

Das Campus-WLAN der CIUP (**WifiCity**) beendet aktive Sitzungen über das Captive Portal regelmäßig, sodass Bewohner mehrmals täglich einen Webbrowser öffnen und ihre Anmeldedaten erneut eingeben müssen.

**CiupWifi** automatisiert diesen Ablauf:
- Nahtlose Authentifizierung im Hintergrund ohne Browserfenster.
- Erfassung der tatsächlichen Sitzungsdauer und vorausschauende Erneuerung der Verbindung vor dem Timeout.
- Ressourcen schonender Betrieb im Infobereich (System Tray) mit minimalem Speicherbedarf (< 15 MB).

---

## Verwendung

1. Verbinden Sie Ihr Gerät mit dem WLAN **WifiCity**.
2. Starten Sie **CiupWifi**, geben Sie Ihre Zugangsdaten ein und klicken Sie auf **Connect**.
3. Schließen Sie das Fenster. Die Anwendung läuft im Hintergrund im System Tray weiter und übernimmt künftige Anmeldungen und Verlängerungen automatisch.

---

## Herunterladen

Offizielle Pakete finden Sie auf der **[Releases-Seite](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Plattform | Datei | Hinweise |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (oder `.msi`) | Standard-Installationsdatei für Windows. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | `CiupWifi.app` in den Programme-Ordner ziehen.<br>*(Bei Gatekeeper-Meldung: Rechtsklick > **Öffnen**)* |
| **Android** | `CiupWifi.apk` | Eigenständiges APK-Paket für Android-Geräte. |
| **iOS / iPadOS** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Über [Orion Browser](https://kagi.com/orion/) oder die Safari-Erweiterung [Userscripts](https://apps.apple.com/app/userscripts/id1463298887) nutzbar. |

---

## Datenschutz & Sicherheit

- **Ausschließlich lokale Speicherung**: Zugangsdaten werden nur lokal auf Ihrem Gerät in Standard-Verzeichnissen gespeichert.
- **Direkte Gateway-Kommunikation**: Anfragen gehen ausschließlich an die interne Portalseite (`10.254.0.254`).
- **Keine Telemetrie**: Keine Tracking-Tools, Statistiken oder externe Server.
- **Vollständig Open Source**: Der Quellcode ist öffentlich einsehbar und prüfbar.

---

## Technische Details & Architektur

Für Entwickler und interessierte Nutzer:

### 1. Direkte native HTTP-Authentifizierung
Statt JavaScript in ein WebView oder einen Browser einzuschleusen, nutzt CiupWifi ein leichtes Rust-Backend über `reqwest`:
- Fragt `http://www.google.com/gen_204` ab, um Captive-Portal-Umleitungen zu erkennen.
- Fängt die HTTP-302-Umleitung zu den FortiGate-Endpunkten ab (`http://10.254.0.254:1000/fgtauth?...`).
- Liest das dynamische Sitzungstoken (`magic`) aus und sendet ein kompatibles Formularpaket (`ft_un`/`username`, `ft_pd`/`password`, `magic`).
- Überprüft den erfolgreichen Verbindungsaufbau über nachfolgende HTTP-204-Rückmeldungen.

### 2. Vorausschauende Sitzungserneuerung
- Filtert kurze Störungen heraus und erfasst nur **Sitzungen ab 1 Stunde Dauer** (maximal die letzten 3 Einträge).
- Nutzt die **kürzeste Dauer (Minimum)** als Richtwert und erneuert die Verbindung 5 Minuten vor diesem Zeitpunkt.
- Vorausschauend erneuerte Verbindungen werden nicht als natürliche Trennung gezählt, damit der Richtwert zuverlässig bleibt.

---

## Referenzen & Danksagung

Dieses Projekt baut auf Vorarbeiten aus der studentischen Entwickler-Community der CIUP auf:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: Ermittelte den FortiGate-Port `1000`, die Rolle des Tokens `magic` und stellte erste Windows- und Linux-Skripte bereit.
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: Entwickler des ursprünglichen Linux-Shell-Skripts zur WifiCity-Anmeldung.

---

## Quellcode kompilieren

### Voraussetzungen
- Node.js 20+
- Rust 1.77+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)

```bash
# Abhängigkeiten installieren
npm install

# Entwicklungsmodus starten
npm run dev

# Release-Pakete erstellen
npm run build
```

---

## Lizenz

Veröffentlicht unter der [MIT-Lizenz](LICENSE).
