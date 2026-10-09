# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icona CiupWifi" />
</p>

<p align="center">
  <b>Accesso automatico e rinnovo per la rete Wi-Fi "WifiCity" della Cité Internationale Universitaire de Paris (CIUP).</b><br>
  <i>Rimani sempre connesso senza dover reinserire le credenziali ad ogni disconnessione.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md"><b>Italiano</b></a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Applicazione non ufficiale**: Utilità open-source indipendente sviluppata da uno studente residente. Non affiliata all'amministrazione o all'IT della CIUP.

---

## 📥 Download (Ultima versione)

Scarica i pacchetti di installazione dalla **[Pagina dei rilasci GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Piattaforma | File di installazione | Istruzioni |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Installer standard. (Se appare SmartScreen: **Ulteriori informazioni** → **Esegui comunque**) |
| **macOS** | `CiupWifi_*_universal.dmg` | Trascina in `Applicazioni`. (Se bloccato: **Impostazioni di Sistema** → **Sicurezza** → **Apri comunque**) |
| **Android** | `CiupWifi.apk` | Pacchetto APK firmato per smartphone e tablet Android. |
| **iOS / iPadOS / Mac** | [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Installazione in 1 clic su Safari ([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) o Orion. |

---

## 🚀 Utilizzo rapido (30 secondi)

1. Connetti il dispositivo alla rete Wi-Fi del campus **WifiCity**.
2. Avvia **CiupWifi**, inserisci **nome utente e password** e clicca su **Connect**.
3. **Puoi chiudere la finestra subito (`[X]`).**
   - Il rinnovo in background viene configurato nell'Utilità di pianificazione del sistema operativo (Windows / macOS).
   - Si attiva in 0,3 s solo quando cade la connessione (**0 MB di RAM a riposo**).

---

## 🛡️ Avvisi di sicurezza al primo avvio (Windows / macOS / Android)

Trattandosi di un'utilità open-source indipendente senza costosi certificati aziendali a pagamento, i sistemi operativi potrebbero mostrare un avviso di sicurezza al primo avvio. Ecco come procedere in 1 secondo in modo sicuro e ufficiale:

* **🪟 Windows (SmartScreen)**:
  - Se appare *"PC protetto da Windows"*: clicca su **[Ulteriori informazioni]** → **[Esegui comunque]**.
* **🍏 macOS (Gatekeeper)**:
  - Se bloccato: clicca su **[Annulla]** → apri **[Impostazioni di Sistema]** → **[Privacy e sicurezza]** → sotto Sicurezza clicca su **[Apri comunque]** → autenticati.
* **🤖 Android (Google Play Protect)**:
  - Se l'installazione viene bloccata: tocca **[Dettagli]** (∨) → tocca **[Installa comunque]** (oppure abilita *Installa app sconosciute* nel browser).
* **🌐 Alternativa senza installazione (Userscript per browser)**:
  - Se preferisci non configurare eccezioni di sicurezza nel sistema, usa il nostro [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) in Safari, Chrome o Firefox. Viene eseguito al 100% all'interno della sandbox del browser.

---

## 🔒 Privacy e Sicurezza

* **100% Locale**: Le credenziali sono salvate solo nella memoria locale del dispositivo.
* **Solo gateway locale**: Comunica solo con il gateway locale del campus (`10.254.0.254`).
* **Open Source**: Nessuna telemetria né tracciamento.

---

## 💻 Compilazione

```bash
npm install
npm run dev      # Modalità sviluppo
npm run build    # Compila pacchetti
```

Licenza MIT
