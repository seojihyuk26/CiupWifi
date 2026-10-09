# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icône CiupWifi" />
</p>

<p align="center">
  <b>Utilitaire de connexion automatique et de maintien de session pour le réseau « WifiCity » de la Cité internationale universitaire de Paris (CIUP).</b>
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
> **Avertissement** : Il s'agit d'un utilitaire open source indépendant développé par des étudiants résidents. Cette application n'est **pas** officielle et n'est en aucun cas affiliée à l'administration ou à la direction des systèmes d'information (DSI) de la Cité internationale universitaire de Paris (CIUP).

---

## Présentation

Le réseau Wi-Fi du campus de la CIUP (**WifiCity**) impose des déconnexions périodiques via son portail captif, obligeant les résidents à rouvrir un navigateur et à ressaisir leurs identifiants plusieurs fois par jour.

**CiupWifi** automatise entièrement ce processus :
- Authentification transparente en arrière-plan sans ouvrir de fenêtre de navigateur.
- Suivi de la durée naturelle des sessions et reconnexion proactive avant que la coupure n'intervienne.
- Fonctionnement discret dans la barre d'état système avec une consommation mémoire minime (< 15 Mo).

---

## Utilisation

1. Connectez votre appareil au réseau Wi-Fi **WifiCity**.
2. Lancez **CiupWifi**, renseignez vos identifiants de campus et cliquez sur **Connect**.
3. Fermez la fenêtre. L'application reste active dans la barre des tâches / zone de notification et gère automatiquement toutes les connexions et reconnexions ultérieures.

---

## Téléchargement

Les paquets d'installation sont disponibles sur la page des **[Versions GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)** :

| Plateforme | Fichier | Remarques |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (ou `.msi`) | Installateur Windows standard. Se lance depuis le menu Démarrer. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Glissez `CiupWifi.app` dans Applications.<br>*(Si Gatekeeper bloque l'ouverture : clic droit > **Ouvrir**)* |
| **Android** | `app-universal-release-unsigned.apk` | Paquet APK autonome pour smartphones et tablettes Android. |
| **iOS / iPadOS** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Utilisable via [Orion Browser](https://kagi.com/orion/) ou l'extension [Userscripts pour Safari](https://apps.apple.com/app/userscripts/id1463298887). |

---

## Confidentialité & Sécurité

- **Stockage strictement local** : Vos identifiants sont sauvegardés uniquement sur votre appareil dans les répertoires standards du système.
- **Communication directe** : Les requêtes sont adressées exclusivement à la passerelle locale du portail (`10.254.0.254`).
- **Aucune télémétrie** : Aucun suivi, statistique ou serveur tiers n'est utilisé.
- **Code source ouvert** : L'ensemble du code est public et vérifiable par tous.

---

## Détails techniques et Architecture

Pour les développeurs et utilisateurs souhaitant comprendre le fonctionnement interne :

### 1. Authentification HTTP native directe
Plutôt que d'injecter du code JavaScript dans une WebView ou un navigateur, CiupWifi exploite un backend Rust léger basé sur `reqwest` :
- Sonde `http://www.google.com/gen_204` pour détecter la redirection du portail captif.
- Intercepte la redirection HTTP 302 vers le portail FortiGate (`http://10.254.0.254:1000/fgtauth?...`).
- Extrait le jeton dynamique de session (`magic`) et soumet les paramètres de formulaire avec double compatibilité (`ft_un`/`username`, `ft_pd`/`password`, `magic`).
- Valide la réussite de la connexion par une requête de contrôle HTTP 204.

### 2. Algorithme de reconnexion prédictive
- Conserve un historique glissant des durées réelles de session (jusqu'à 30 enregistrements).
- Calcule la durée minimale observée ($T_{\text{min}}$).
- Déclenche une reconnexion anticipée à $T_{\text{min}} - T_{\text{écoulé}} - 5\text{ min}$, évitant ainsi les coupures inattendues pendant le travail ou le streaming.
- Les reconnexions initiées proactivement sont exclues des statistiques de déconnexion naturelle afin de préserver la fiabilité du calcul.

---

## Références et Remerciements

Ce projet s'appuie sur les travaux d'analyse menés par la communauté étudiante de la CIUP :
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)** : Travail remarquable ayant identifié le port `1000` de FortiGate, le rôle du jeton `magic` et proposé les premiers scripts CLI pour Windows et Linux.
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)** : Auteur du script shell Linux d'origine pour l'automatisation du portail WifiCity.

---

## Compilation depuis les sources

### Prérequis
- Node.js 20+
- Rust 1.77+
- Tauri CLI v2 (`npm install -g @tauri-apps/cli`)

```bash
# Installation des dépendances
npm install

# Lancement en mode développement
npm run dev

# Compilation des exécutables finaux
npm run build
```

---

## Licence

Distribué sous licence libre [MIT](LICENSE).
