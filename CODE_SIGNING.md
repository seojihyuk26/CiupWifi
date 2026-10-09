# Code Signing Policy

CiupWifi builds are signed to ensure authenticity, integrity, and safety for users running our software.

## Open Source & Transparency
* **Repository**: [https://github.com/seojihyuk26/CiupWifi](https://github.com/seojihyuk26/CiupWifi)
* **License**: MIT License
* **Releases**: [https://github.com/seojihyuk26/CiupWifi/releases](https://github.com/seojihyuk26/CiupWifi/releases)

## Signing Pipeline
* Official releases are built from clean Git tags exclusively via GitHub Actions CI/CD workflows (`.github/workflows/release.yml`).
* No binary is signed manually on developer machines.
* All artifacts and build outputs are verifiable against the public GitHub source code repository.
* Windows binaries are signed via SignPath Foundation to protect students and campus residents against malware spoofing and untrusted binary alerts.

