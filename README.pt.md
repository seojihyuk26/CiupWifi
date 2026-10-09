# CiupWifi 📶

<p align="center">
  <img src="app-icon.svg" width="128" height="128" alt="Ícone CiupWifi" />
</p>

<p align="center">
  <b>Login automático e renovação de conexão para a rede Wi-Fi "WifiCity" da Cité Internationale Universitaire de Paris (CIUP).</b><br>
  <i>Mantenha-se conectado sem precisar digitar suas credenciais repetidamente.</i>
</p>

<p align="center">
  <a href="README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.it.md">Italiano</a> •
  <a href="README.pt.md"><b>Português</b></a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a>
</p>

> [!WARNING]
> **Aplicativo não oficial**: Utilitário de código aberto independente desenvolvido por um estudante residente, sem afiliação com a administração ou TI da CIUP.

---

## 📥 Downloads (Versão mais recente)

Baixe os pacotes na **[Página de Releases do GitHub](https://github.com/seojihyuk26/CiupWifi/releases/latest)**:

| Plataforma | Instalador | Instruções |
|---|---|---|
| **Windows** | `CiupWifi_*_x64-setup.exe` | Instalador padrão. (Se o SmartScreen aparecer: **Mais informações** → **Executar assim mesmo**) |
| **macOS** | `CiupWifi_*_universal.dmg` | Arraste para `Aplicativos`. (Se bloqueado: **Ajustes do Sistema** → **Segurança** → **Abrir Mesmo Assim**) |
| **Android** | `CiupWifi.apk` | Arquivo APK assinado para celulares e tablets Android. |
| **iOS / iPadOS / Mac** | [**Userscript Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) | Instalação em 1 clique no Safari ([Userscripts](https://apps.apple.com/app/userscripts/id1463298887)) ou Orion. |

---

## 🚀 Uso rápido (30 segundos)

1. Conecte seu dispositivo à rede Wi-Fi **WifiCity**.
2. Abra o **CiupWifi**, digite seu **usuário e senha** e clique em **Connect**.
3. **Pode fechar a janela imediatamente (`[X]`).**
   - A renovação automática em segundo plano é registrada no Agendador de Tarefas do seu sistema (Windows / macOS).
   - Executa silenciosamente em 0,3 s apenas quando desconectado (**0 MB de memória em repouso**).

---

## 🛡️ Avisos de segurança no primeiro início (Windows / macOS / Android)

Por se tratar de um utilitário de código aberto sem certificados empresariais pagos, os sistemas operacionais podem exibir avisos no primeiro início. Prossiga com segurança em 1 segundo pelo procedimento oficial do seu sistema:

* **🪟 Windows (SmartScreen)**:
  - Se o aviso *"O Windows protegeu o seu PC"* aparecer: clique em **[Mais informações]** → **[Executar assim mesmo]**.
* **🍏 macOS (Gatekeeper)**:
  - Se bloqueado: clique em **[Cancelar]** → abra **[Ajustes do Sistema]** → **[Privacidade e Segurança]** → sob Segurança, clique em **[Abrir Mesmo Assim]** → autentique com Touch ID ou senha.
* **🤖 Android (Google Play Protect)**:
  - Se a instalação for bloqueada: toque em **[Mais detalhes]** (∨) → toque em **[Instalar mesmo assim]** (ou permita *Instalar aplicativos desconhecidos* no seu navegador).
* **🌐 Alternativa sem instalação (Userscript no Navegador)**:
  - Se preferir não configurar exceções de segurança no sistema, use o nosso [**Userscript no Greasy Fork**](https://greasyfork.org/fr/scripts/488569-cite-university-wifi-auto-login-script?locale_override=1) no Safari, Chrome ou Firefox. Ele roda 100% isolado dentro da sandbox do navegador.

---

## 🔒 Privacidade e Segurança

* **100% Local**: As credenciais ficam salvas exclusivamente no armazenamento local do seu dispositivo.
* **Apenas gateway local**: Comunica-se unicamente com o gateway da residência (`10.254.0.254`).
* **Código aberto**: Sem telemetria ou rastreamento.

---

## 💻 Compilação

```bash
npm install
npm run dev      # Modo desenvolvimento
npm run build    # Compilar pacotes
```

Licença MIT
