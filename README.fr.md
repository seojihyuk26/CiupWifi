# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icône CiupWifi" />
</p>

<p align="center">
  <b>Connexion automatique et reconnexion proactive pour le Wi-Fi de la Cité internationale universitaire de Paris (CIUP).</b>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md"><b>Français</b></a> •
  <a href="README.es.md">Español</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.de.md">Deutsch</a>
</p>

> [!WARNING]
> **Avertissement important** : Cette application est un **outil open source non officiel** développé par des étudiants résidents. Elle n'est **EN AUCUN CAS** développée, approuvée, gérée ou affiliée à l'administration ou aux services informatiques de la **Cité internationale universitaire de Paris (CIUP)**.

---

## 💡 Qu'est-ce que CiupWifi ?

À la Cité internationale universitaire de Paris (CIUP), le portail captif déconnecte régulièrement les sessions, obligeant les résidents à saisir leurs identifiants plusieurs fois par jour.

**CiupWifi** est une application native multiplateforme (Windows, macOS, Android) développée avec **Tauri v2** qui automatise entièrement ce processus :
- **Aucun navigateur requis** : Authentification directe auprès du portail `10.254.0.254` via des requêtes HTTP natives — aucune fenêtre de navigateur intempestive ni WebView.
- **Reconnexion proactive prédictive** : Analyse la durée moyenne de vos sessions et anticipe l'expiration pour vous reconnecter automatiquement quelques minutes *avant* la coupure.
- **Léger et économe en batterie** : Reste discrètement dans la barre d'état système (System Tray) avec une consommation mémoire minimale (< 15 Mo).
- **Confidentialité totale** : Vos identifiants sont stockés exclusivement sur votre appareil local et ne quittent jamais votre machine.

---

## 📥 Téléchargement & Installation

Rendez-vous sur la page des **[Dernières versions GitHub](https://github.com/seojihyuk/CiupWifi/releases/latest)** pour télécharger la version adaptée à votre système :

| Plateforme | Fichier | Instructions |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (ou `.msi`) | Lancez l'installateur et ouvrez l'application depuis le menu Démarrer. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Ouvrez le `.dmg` et glissez `CiupWifi.app` dans le dossier Applications.<br>*(Si macOS bloque l'ouverture pour développeur non identifié : faites un clic droit sur l'application > **Ouvrir**)* |
| **Android** | `app-universal-release-unsigned.apk` | Téléchargez et installez le fichier APK. Autorisez l'installation d'applications de sources inconnues si demandé. |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Utilisez [Orion Browser](https://kagi.com/orion/) ou l'extension [Userscripts pour Safari](https://apps.apple.com/app/userscripts/id1463298887) et installez `wifiLogin.js`. |

---

## 🚀 Guide d'utilisation

1. **Connectez-vous** au réseau Wi-Fi de la CIUP.
2. **Lancez CiupWifi**.
3. Saisissez votre **Identifiant** et votre **Mot de passe** de campus une seule fois, puis cliquez sur **Connect**.
4. C'est tout ! 
   - Sur ordinateur : Vous pouvez fermer la fenêtre, l'application reste active dans la barre d'état.
   - CiupWifi se reconnectera automatiquement en tâche de fond avant chaque expiration de session.

---

## 🔒 Sécurité & Vie privée

- **Aucun serveur distant** : CiupWifi ne collecte aucune donnée télémétrique ni statistique externe.
- **Stockage localisé** : Les identifiants sont sauvegardés uniquement dans le dossier de configuration local de l'application.
- **Open Source** : L'intégralité du code source est ouverte et vérifiable par tous.

---

## 📄 Licence & Mentions légales

Distribué sous licence [MIT](LICENSE).  
**Application non officielle** : CiupWifi est un utilitaire open-source indépendant créé par des résidents pour des résidents, et n'est en aucun cas affilié, soutenu ou associé à la Cité internationale universitaire de Paris (CIUP).
