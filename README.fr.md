# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icône CiupWifi" />
</p>

<p align="center">
  <b>Connexion automatique au réseau Wi-Fi "WifiCity" de la Cité internationale universitaire de Paris (CIUP).</b><br>
  <i>Restez connecté sans devoir retaper vos identifiants à longueur de journée.</i>
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
> **Information importante (Application non officielle)** : Il s'agit d'un utilitaire communautaire indépendant développé par un étudiant résident. Cette application n'est **PAS** officielle et n'est en aucun cas affiliée à l'administration ou à la DSI de la Cité internationale universitaire de Paris (CIUP).

---

## 🤔 À quoi sert cette application ? (Objectif)

Si vous habitez à la CIUP et utilisez le Wi-Fi de la cité (**WifiCity**), vous connaissez bien ce problème :
- Toutes les quelques heures, la session expire et la connexion se coupe.
- Vous devez ouvrir votre navigateur, attendre que la page de connexion s'affiche et retaper votre identifiant et votre mot de passe.
- En plein appel vidéo, révision ou streaming, internet se coupe brutalement.

**CiupWifi résout ce problème une bonne fois pour toutes.**  
Une fois installée, l'application tourne discrètement en arrière-plan sur votre ordinateur ou votre téléphone. Elle vous connecte automatiquement au Wi-Fi et anticipe l'expiration pour renouveler votre connexion *avant* qu'elle ne soit coupée.

---

## 🚀 Comment l'utiliser ? (En 3 étapes simples)

La configuration ne prend que **30 secondes**, une seule fois :

1. **Connectez-vous** au réseau Wi-Fi de la résidence nommé **WifiCity**.
2. **Ouvrez l'application CiupWifi**, entrez votre **Identifiant** et votre **Mot de passe** de campus, puis cliquez sur **Connect**.
3. **C'est terminé !** Vous pouvez fermer la fenêtre.
   - Sur PC/Mac, l'application reste active discrètement dans la barre des tâches (icône près de l'horloge).
   - Dès que votre session s'apprête à expirer, l'application la renouvelle automatiquement sans vous déranger.

---

## 📥 Téléchargement

Téléchargez la version correspondant à votre appareil depuis la **[Page des versions GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)** :

| Votre appareil | Fichier à télécharger | Installation facile |
|---|---|---|
| **Ordinateur Windows** | `CiupWifi_x.x.x_x64-setup.exe` | Téléchargez, double-cliquez pour installer et lancez l'application. |
| **Mac (Apple)** | `CiupWifi_x.x.x_universal.dmg` | Ouvrez le fichier et glissez `CiupWifi` dans votre dossier Applications.<br>*(Si un message de sécurité apparaît : clic droit sur l'icône > **Ouvrir**)* |
| **Téléphone Android** | `app-universal-release-unsigned.apk` | Téléchargez et installez le fichier APK sur votre smartphone. |
| **iPhone / iPad** | [`wifiLogin.js`](wifiLogin.js) (Script) | Installez [Orion Browser](https://kagi.com/orion/) ou l'extension [Userscripts pour Safari](https://apps.apple.com/app/userscripts/id1463298887) et ajoutez le script `wifiLogin.js`. |

---

## 🔒 Mon mot de passe est-il en sécurité ?

**Oui, absolument.**
- Vos identifiants sont enregistrés **uniquement sur votre propre appareil**.
- Aucune donnée n'est envoyée à des serveurs tiers ou au créateur de l'application.
- L'application communique exclusivement et directement avec la page de connexion de la cité (`10.254.0.254`).
- Le projet est entièrement gratuit, open-source et transparent.

---

## 🙏 Remerciements et Références

Ce projet s'inspire directement des travaux et analyses préalables menés par la communauté étudiante de la CIUP :
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)** : Travail remarquable sur les scripts d'automatisation Windows et Linux ayant mis en lumière le port `1000` et les jetons de sécurité `magic` de FortiGate.
- **[Gist de Ranadeep Biswas](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)** : Script shell Linux original pour la connexion WifiCity.

CiupWifi transforme ces découvertes techniques en une application graphique moderne et accessible à tous les résidents.

---

## 📄 Licence

Distribué sous licence libre [MIT](LICENSE).
