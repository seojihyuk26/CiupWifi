# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Icono de CiupWifi" />
</p>

<p align="center">
  <b>Inicio de sesión automático y renovación para la red Wi-Fi "WifiCity" de la Cité Internationale Universitaire de Paris (CIUP).</b><br>
  <i>Mantente conectado sin tener que introducir tus credenciales repetidamente.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md"><b>Español</b></a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Aplicación no oficial**: Utilidad independiente de código abierto desarrollada por un estudiante residente. No tiene afiliación con la administración o TI de la CIUP.

---

## 📥 Descargas (Última versión)

Descarga los paquetes desde **[GitHub Releases](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Plataforma | Instalador | Instrucciones |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Instalador estándar. (Si aparece SmartScreen: **Más información** → **Ejecutar de todas formas**) |
| **macOS** | `CiupWifi_*_universal.dmg` | Arrastra a `Aplicaciones`. (Si se bloquea: **Ajustes del Sistema** → **Seguridad** → **Abrir de todos modos**) |
| **Android** | `CiupWifi.apk` | Archivo APK firmado para teléfonos y tabletas. |
| **iOS / iPadOS / Mac** | [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Instalación en 1 clic en Safari ([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) u Orion. |

---

## 🚀 Uso rápido (30 segundos)

1. Conecta tu dispositivo a la red Wi-Fi **WifiCity**.
2. Abre **CiupWifi**, introduce tu **usuario y contraseña**, y haz clic en **Connect**.
3. **Puedes cerrar la ventana inmediatamente (`[X]`).**
   - La renovación en segundo plano se registra automáticamente en el Programador de tareas de tu sistema operativo.
   - Solo se ejecuta en 0,3 s cuando la red se corta, consumiendo **0 MB de memoria en reposo**.

---

## 🛡️ Avisos de seguridad en el primer inicio (Windows / macOS / Android)

Como utilidad independiente de código abierto sin certificados empresariales de pago, los sistemas operativos pueden mostrar advertencias en el primer inicio. Sigue el procedimiento oficial según tu sistema:

* **🪟 Windows (SmartScreen)**:
  - Si aparece *"Windows protegió su PC"*: haz clic en **[Más información]** → **[Ejecutar de todas formas]**.
* **🍏 macOS (Gatekeeper)**:
  - Si se bloquea: haz clic en **[Cancelar]** → abre **[Ajustes del Sistema]** → **[Privacidad y seguridad]** → bajo Seguridad, haz clic en **[Abrir de todos modos]** → autentícate.
* **🤖 Android (Google Play Protect)**:
  - Si se bloquea la instalación: pulsa **[Detalles]** (∨) → pulsa **[Instalar de todas formas]** (o habilita *Instalar aplicaciones desconocidas* en tu navegador).
* **🌐 Alternativa sin instalación (Userscript de navegador)**:
  - Si prefieres no configurar excepciones de seguridad, utiliza nuestro [**Userscript en Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) en Safari, Chrome o Firefox. Se ejecuta 100% dentro del entorno aislado del navegador.

---

## 🔒 Privacidad y Seguridad

* **100% Local**: Las credenciales se almacenan únicamente en tu dispositivo.
* **Solo pasarela local**: Se comunica exclusivamente con el gateway de autenticación del campus (`10.254.0.254`).
* **Código abierto**: Sin telemetría ni rastreo.

---

## 💻 Compilación

```bash
npm install
npm run dev      # Modo desarrollo
npm run build    # Compilar paquetes
```

Licencia MIT
