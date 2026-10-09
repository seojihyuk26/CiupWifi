# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="CiupWifi Icon" />
</p>

<p align="center">
  <b>Automatische Anmeldung für das "WifiCity"-WLAN der Cité internationale universitaire de Paris (CIUP).</b><br>
  <i>Bleiben Sie online, ohne ständig Benutzername und Passwort eingeben zu müssen.</i>
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
> **Wichtiger Hinweis (Inoffizielles Tool)**: Dies ist eine unabhängige, von Bewohnern erstellte Hilfsanwendung. Es handelt sich **NICHT** um eine offizielle App der Cité internationale universitaire de Paris (CIUP).

---

## 🤔 Wozu dient diese App? (Zweck)

Jeder, der an der CIUP wohnt und das Campus-WLAN (**WifiCity**) nutzt, kennt das Problem:
- Alle paar Stunden läuft die Sitzung ab und das Internet bricht plötzlich ab.
- Man muss einen Browser öffnen, warten, bis die Login-Seite lädt, und die Zugangsdaten erneut eingeben.
- Beim Lernen, Streamen oder in Videoanrufen wird die Verbindung mitten im Satz unterbrochen.

**CiupWifi löst dieses Problem dauerhaft.**  
Einmal installiert, läuft die App unauffällig im Hintergrund auf Ihrem Computer oder Smartphone. Sie verbindet Sie automatisch mit dem WLAN und erneuert die Verbindung selbstständig *bevor* sie abbricht.

---

## 🚀 Einfache Bedienung (In 3 Schritten)

Die Einrichtung ist in **wenigen Sekunden** erledigt:

1. Verbinden Sie Ihr Gerät mit dem WLAN **WifiCity**.
2. **Öffnen Sie CiupWifi**, geben Sie Ihren **Benutzernamen** und Ihr **Passwort** ein und klicken Sie auf **Connect**.
3. **Fertig!** Sie können das Fenster schließen.
   - Auf PC/Mac bleibt die App im Infobereich (System Tray neben der Uhr) aktiv.
   - Bevor Ihre Sitzung abläuft, verlängert CiupWifi die Verbindung automatisch im Hintergrund.

---

## 📥 Herunterladen

Laden Sie die passende Version auf der **[GitHub Releases-Seite](https://github.com/seojihyuk26/CiupWifi/releases/latest)** herunter:

| Ihr Gerät | Datei | Einfache Installation |
|---|---|---|
| **Windows PC** | `CiupWifi_x.x.x_x64-setup.exe` | Herunterladen, doppelklicken und installieren. |
| **Mac (Apple)** | `CiupWifi_x.x.x_universal.dmg` | Datei öffnen und `CiupWifi` in den Programme-Ordner ziehen.<br>*(Bei Sicherheitshinweis: Rechtsklick > **Öffnen**)* |
| **Android Smartphone** | `app-universal-release-unsigned.apk` | APK-Datei auf das Handy herunterladen und installieren. |
| **iPhone / iPad** | [`wifiLogin.js`](wifiLogin.js) (Skript) | [Orion Browser](https://kagi.com/orion/) oder die Safari-Erweiterung [Userscripts](https://apps.apple.com/app/userscripts/id1463298887) installieren und das Skript `wifiLogin.js` hinzufügen. |

---

## 🔒 Sind meine Daten sicher?

**Ja, zu 100%.**
- Ihre Zugangsdaten werden **ausschließlich lokal auf Ihrem eigenen Gerät** gespeichert.
- Es werden keinerlei Daten an externe Server oder Entwickler übertragen.
- Die App kommuniziert ausschließlich direkt mit der internen Login-Seite (`10.254.0.254`).
- Das gesamte Projekt ist quelloffen (Open Source), kostenlos und transparent.

---

## 🙏 Danksagung & Referenzen

Dieses Projekt basiert auf wertvollen Vorarbeiten aus der Bewohner-Community der CIUP:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: Großartige Automatisierungsskripte für Windows und Linux, die den Port `1000` und die FortiGate-Sicherheitstoken (`magic`) entschlüsselt haben.
- **[Gist von Ranadeep Biswas](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: Das ursprüngliche Shell-Skript für Linux zur automatischen WifiCity-Anmeldung.

CiupWifi bündelt diese technischen Erkenntnisse in einer modernen grafischen App für den unkomplizierten Alltag aller Studierenden.

---

## 📄 Lizenz

Veröffentlicht unter der [MIT-Lizenz](LICENSE).
