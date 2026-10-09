# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icono de CiupWifi" />
</p>

<p align="center">
  <b>Utilidad de inicio de sesión automático y mantenimiento de conexión para la red «WifiCity» de la Cité internationale universitaire de Paris (CIUP).</b>
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
> **Aviso**: Esta es una herramienta independiente de código abierto creada por estudiantes residentes. **No** es una aplicación oficial ni cuenta con afiliación o respaldo de la administración o los servicios informáticos de la Cité internationale universitaire de Paris (CIUP).

---

## Descripción general

La red Wi-Fi de la CIUP (**WifiCity**) desconecta periódicamente las sesiones activas mediante su portal cautivo, lo que obliga a los residentes a abrir el navegador y reautenticarse varias veces al día.

**CiupWifi** automatiza este proceso:
- Autenticación en segundo plano sin necesidad de abrir ventanas del navegador.
- Monitoreo de la duración natural de las sesiones y reconexión preventiva antes de que ocurra el corte.
- Funcionamiento discreto en la bandeja del sistema con un consumo de memoria mínimo (< 15 MB).

---

## Uso

1. Conecta tu dispositivo a la red Wi-Fi **WifiCity**.
2. Abre **CiupWifi**, introduce tus credenciales del campus y haz clic en **Connect**.
3. Cierra la ventana. La aplicación continuará activa en la barra de tareas y gestionará los inicios de sesión y renovaciones automáticamente.

---

## Descargas

Los paquetes de instalación están disponibles en la sección de **[Versiones en GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Plataforma | Paquete | Notas |
|---|---|---|
| **Windows** | `CiupWifi_x.x.x_x64-setup.exe` (o `.msi`) | Instalador estándar de Windows. Ejecutable desde el menú Inicio. |
| **macOS** | `CiupWifi_x.x.x_universal.dmg` | Arrastra `CiupWifi.app` a Aplicaciones.<br>*(Si Gatekeeper muestra advertencia: clic derecho > **Abrir**)* |
| **Android** | `app-universal-release-unsigned.apk` | Paquete APK independiente para dispositivos Android. |
| **iOS / iPadOS** | [`wifiLogin.js`](wifiLogin.js) (Userscript) | Utilizable mediante [Orion Browser](https://kagi.com/orion/) o la extensión [Userscripts para Safari](https://apps.apple.com/app/userscripts/id1463298887). |

---

## Privacidad y Seguridad

- **Almacenamiento exclusivamente local**: Tus contraseñas se guardan únicamente en tu dispositivo en el directorio estándar de la aplicación.
- **Comunicación directa con la pasarela**: Las peticiones se envían únicamente a la dirección local del portal (`10.254.0.254`).
- **Sin telemetría**: No incluye análisis, rastreo ni servidores de terceros.
- **Código abierto**: Todo el código fuente está disponible públicamente para su revisión.

---

## Detalles técnicos y Arquitectura

Para desarrolladores y usuarios interesados en el funcionamiento interno:

### 1. Autenticación HTTP nativa directa
En lugar de inyectar scripts en un WebView o navegador, CiupWifi utiliza un backend en Rust con `reqwest`:
- Consulta `http://www.google.com/gen_204` para detectar la redirección del portal cautivo.
- Intercepta la redirección HTTP 302 hacia los extremos de FortiGate (`http://10.254.0.254:1000/fgtauth?...`).
- Extrae el token dinámico de sesión (`magic`) y envía los datos con doble compatibilidad de parámetros (`ft_un`/`username`, `ft_pd`/`password`, `magic`).
- Comprueba el éxito de la conexión mediante una solicitud de verificación HTTP 204.

### 2. Algoritmo de renovación predictiva
- Registra el historial de duraciones reales de sesión (hasta 30 registros).
- Determina la duración mínima observada ($T_{\text{mín}}$).
- Programa la reconexión automática a $T_{\text{mín}} - T_{\text{transcurrido}} - 5\text{ min}$, evitando desconexiones repentinas durante el trabajo o la transmisión de vídeo.
- Las reconexiones proactivas se excluyen de las estadísticas de expiración natural para mantener la precisión del cálculo.

---

## Referencias y Créditos

Este proyecto se basa en investigaciones y herramientas previas de la comunidad de estudiantes de la CIUP:
- **[Thomas-dd3/WifiCity_captiveportal](https://github.com/Thomas-dd3/WifiCity_captiveportal)**: Identificó el puerto `1000` de FortiGate, el mecanismo del token `magic` y ofreció los primeros scripts de terminal para Windows y Linux.
- **[Ranadeep Biswas (rnbguy)](https://gist.github.com/rnbguy/6f574caa6b3535162a20750cb1777a09)**: Creador del script shell de Linux original para la conexión en WifiCity.

---

## Licencia

Distribuido bajo la [Licencia MIT](LICENSE).
