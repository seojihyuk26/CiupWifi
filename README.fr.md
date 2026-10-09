# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icône CiupWifi" />
</p>

<p align="center">
  <b>Connexion automatique et maintien de session pour le réseau Wi-Fi "WifiCity" de la Cité internationale universitaire de Paris (CIUP).</b><br>
  <i>Restez connecté sans devoir ressaisir vos identifiants à chaque coupure.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md"><b>Français</b></a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Application non officielle** : Développée de manière indépendante par un étudiant résident, sans affiliation avec l'administration ou la DSI de la CIUP.

---

## 📥 Téléchargement (Dernière version)

Téléchargez l'application depuis la page **[Releases GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)** :

| Plateforme | Fichier | Instructions |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Exécutez l'installateur. (Si SmartScreen apparaît : **Informations complémentaires** → **Exécuter quand même**) |
| **macOS** | `CiupWifi_*_universal.dmg` | Glissez dans `Applications`. (Si bloqué : **Réglages Système** → **Sécurité** → **Ouvrir quand même**) |
| **Android** | `CiupWifi.apk` | Fichier APK signé pour smartphones et tablettes. |
| **iOS / iPadOS / Mac** | [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Installation en 1 clic dans Safari ([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) ou Orion. |

---

## 🚀 Utilisation (En 30 secondes)

1. Connectez-vous au Wi-Fi **WifiCity**.
2. Ouvrez **CiupWifi**, entrez votre **identifiant/mot de passe** du campus, puis cliquez sur **Connect**.
3. **Vous pouvez fermer la fenêtre (`[X]`).**
   - Le renouvellement automatique est configuré dans le planificateur de tâches de l'OS (Windows / macOS).
   - Ne reste pas en tâche de fond inutilement : s'exécute en 0,3s seulement quand nécessaire (**0 Mo de RAM au repos**).

---

## 🛡️ Avertissements de sécurité au premier lancement (Windows / macOS / Android)

En tant qu'utilitaire open-source indépendant sans certificat d'entreprise payant, les systèmes d'exploitation peuvent afficher un avertissement au premier lancement. Voici la procédure officielle pour chaque système :

* **🪟 Windows (SmartScreen)** :
  - Si *"Windows a protégé votre ordinateur"* s'affiche : cliquez sur **[Informations complémentaires]** → **[Exécuter quand même]**.
* **🍏 macOS (Gatekeeper)** :
  - Si bloqué : cliquez sur **[Annuler]** → ouvrez **[Réglages Système]** → **[Confidentialité et sécurité]** → sous Sécurité, cliquez sur **[Ouvrir quand même]** → authentifiez-vous.
* **🤖 Android (Google Play Protect)** :
  - Si l'installation est bloquée : touchez **[Détails]** (∨) → touchez **[Installer quand même]** (ou autorisez *Installation d'applications inconnues* dans votre navigateur).
* **🌐 Alternative sans installation (Userscript navigateur)** :
  - Si vous préférez ne pas configurer d'exceptions de sécurité, utilisez notre [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) dans Safari, Chrome ou Firefox. Il s'exécute à 100% dans le bac à sable du navigateur.

---

## 🔒 Confidentialité & Sécurité

* **100% Local** : Vos identifiants restent stockés uniquement sur votre machine.
* **Passerelle locale uniquement** : Communique uniquement avec la passerelle locale du campus (`10.254.0.254`).
* **Open Source** : Aucune télémétrie ni collecte de données.

---

## 💻 Compilation

```bash
npm install
npm run dev      # Mode développement
npm run build    # Compiler les paquets
```

Licence MIT
