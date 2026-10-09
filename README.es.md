# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icono de CiupWifi" />
</p>

<p align="center">
  <b>Inicio de sesión automático y reconexión proactiva para la red Wi-Fi de la Cité internationale universitaire de Paris (CIUP).</b>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md"><b>Español</b></a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.de.md">Deutsch</a>
</p>

> [!WARNING]
> **Descargo de responsabilidad importante**: Esta es una herramienta de código abierto **NO OFICIAL** desarrollada por residentes para residentes. **NO está** desarrollada, respaldada ni afiliada de ninguna manera con la administración o los servicios informáticos de la **Cité internationale universitaire de Paris (CIUP)**.

---

## 💡 ¿Qué es CiupWifi?

En la Cité internationale universitaire de Paris (CIUP), el portal cautivo expira con frecuencia las sesiones activas, obligando a los residentes a ingresar sus credenciales repetidamente cada día.

**CiupWifi** es una aplicación nativa multiplataforma (Windows, macOS, Android) creada con **Tauri v2** que automatiza por completo este proceso:
- **Sin necesidad de navegador**: Inicia sesión directamente en `10.254.0.254` mediante solicitudes HTTP nativas — sin molestas ventanas de navegador ni WebViews.
- **Reconexión proactiva predictiva**: Registra la duración habitual de tus sesiones y predice la desconexión, reconectándose minutos *antes* de que se corte la red.
- **Ligera y eficiente con la batería**: Funciona silenciosamente en la bandeja del sistema (System Tray) con un consumo de memoria mínimo (< 15 MB).
- **Privacidad total**: Tus credenciales se almacenan únicamente en tu dispositivo local y nunca salen de tu ordenador.

---

## 📥 Descarga e Instalación

Accede a la sección de **[Últimas versiones en GitHub](https://github.com/seojihyuk/CiupWifi/releases/latest)** para descargar el instalador de tu dispositivo:

| Plataforma | Archivo | Instrucciones |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (o `.msi`) | Ejecuta el instalador e inicia la aplicación desde el menú Inicio. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Abre el archivo `.dmg` y arrastra `CiupWifi.app` a Aplicaciones.<br>*(Si macOS indica desarrollador no identificado: clic derecho > **Abrir**)* |
| **Android** | `app-universal-release-unsigned.apk` | Descarga e instala el archivo APK. Permite orígenes desconocidos si es necesario. |
| **iOS / iPhone** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Utiliza [Orion Browser](https://kagi.com/orion/) o [Userscripts para Safari](https://apps.apple.com/app/userscripts/id1463298887) e instala `wifiLogin.js`. |

---

## 🚀 Instrucciones de uso

1. **Conéctate** a la red Wi-Fi de la CIUP.
2. **Abre CiupWifi**.
3. Ingresa tu **Usuario** y **Contraseña** del campus una sola vez y haz clic en **Connect**.
4. ¡Listo! 
   - En ordenador: Puedes cerrar la ventana; permanecerá activa en la bandeja del sistema.
   - CiupWifi se reconectará de forma automática antes de cada expiración.

---

## 🔒 Seguridad y Privacidad

- **Sin servidores remotos**: CiupWifi no recopila datos de telemetría ni envía información a terceros.
- **Almacenamiento local**: Tus contraseñas se guardan de forma segura en la carpeta de datos de la aplicación de tu propio sistema.
- **Código abierto**: Todo el código fuente está disponible públicamente para su inspección.

---

## 📄 Licencia y Nota Legal

Distribuido bajo la [Licencia MIT](LICENSE).  
**Aplicación no oficial**: CiupWifi es una herramienta comunitaria independiente y no tiene ninguna relación oficial con la Cité internationale universitaire de Paris (CIUP).
