#!/usr/bin/env python3
"""Applies the zero-permission event-driven Android overlay to Tauri's generated Android project."""

import json
from pathlib import Path
import re
import shutil
import sys
import xml.etree.ElementTree as ET

ANDROID_NS = "http://schemas.android.com/apk/res/android"
ET.register_namespace("android", ANDROID_NS)

# Minimal permissions: zero Location, zero Notification, zero Foreground Service
REQUIRED_PERMISSIONS = [
    "android.permission.INTERNET",
    "android.permission.ACCESS_NETWORK_STATE",
    "android.permission.CHANGE_NETWORK_STATE",
    "android.permission.RECEIVE_BOOT_COMPLETED",
]


def android_attr(name: str) -> str:
    return f"{{{ANDROID_NS}}}{name}"


def patch_manifest(manifest_path: Path) -> None:
    tree = ET.parse(manifest_path)
    root = tree.getroot()

    existing_perms = {
        el.get(android_attr("name"))
        for el in root.findall("uses-permission")
    }

    app_el = root.find("application")
    if app_el is None:
        raise RuntimeError(f"<application> element missing in {manifest_path}")

    insert_idx = list(root).index(app_el)
    for perm in REQUIRED_PERMISSIONS:
        if perm not in existing_perms:
            el = ET.Element("uses-permission", {android_attr("name"): perm})
            root.insert(insert_idx, el)
            insert_idx += 1

    app_el.set(android_attr("usesCleartextTraffic"), "true")
    app_el.set(android_attr("networkSecurityConfig"), "@xml/network_security_config")

    # Receiver: .WifiEventReceiver (OS-triggered ConnectivityManager PendingIntent)
    if not any(
        el.get(android_attr("name")) in (".WifiEventReceiver", "org.ciup.wifi.WifiEventReceiver")
        for el in app_el.findall("receiver")
    ):
        ET.SubElement(
            app_el,
            "receiver",
            {
                android_attr("name"): ".WifiEventReceiver",
                android_attr("enabled"): "true",
                android_attr("exported"): "false",
            },
        )

    # Service: .WifiAuthJobService (JobScheduler captive-network wake trigger, API 28+)
    if not any(
        el.get(android_attr("name")) in (".WifiAuthJobService", "org.ciup.wifi.WifiAuthJobService")
        for el in app_el.findall("service")
    ):
        ET.SubElement(
            app_el,
            "service",
            {
                android_attr("name"): ".WifiAuthJobService",
                android_attr("permission"): "android.permission.BIND_JOB_SERVICE",
                android_attr("enabled"): "true",
                android_attr("exported"): "false",
            },
        )

    # Receiver: .BootReceiver (re-registers triggers after device reboot / app update)
    if not any(
        el.get(android_attr("name")) in (".BootReceiver", "org.ciup.wifi.BootReceiver")
        for el in app_el.findall("receiver")
    ):
        receiver_el = ET.SubElement(
            app_el,
            "receiver",
            {
                android_attr("name"): ".BootReceiver",
                android_attr("enabled"): "true",
                android_attr("exported"): "false",
            },
        )
        intent_filter = ET.SubElement(receiver_el, "intent-filter")
        ET.SubElement(intent_filter, "action", {android_attr("name"): "android.intent.action.BOOT_COMPLETED"})
        ET.SubElement(intent_filter, "action", {android_attr("name"): "android.intent.action.MY_PACKAGE_REPLACED"})

    ET.indent(tree, space="    ")
    tree.write(manifest_path, encoding="utf-8", xml_declaration=True)


def patch_proguard(proguard_path: Path) -> None:
    rule = "-keepclassmembers class org.ciup.wifi.WifiBridge {\n    <methods>;\n}\n"
    existing = proguard_path.read_text(encoding="utf-8") if proguard_path.exists() else ""
    if "org.ciup.wifi.WifiBridge" not in existing:
        if existing and not existing.endswith("\n"):
            existing += "\n"
        proguard_path.write_text(existing + "\n" + rule, encoding="utf-8")


def compute_version_code(repo_root: Path) -> tuple[int, str]:
    conf_path = repo_root / "src-tauri" / "tauri.conf.json"
    data = json.loads(conf_path.read_text(encoding="utf-8"))
    version_name = str(data.get("version", "1.0.0")).strip()
    core = version_name.split("-")[0]
    parts = [int(p) for p in core.split(".")[:3]]
    while len(parts) < 3:
        parts.append(0)
    version_code = parts[0] * 10000 + parts[1] * 100 + parts[2]
    return version_code, version_name


def patch_gradle_config(repo_root: Path) -> None:
    version_code, version_name = compute_version_code(repo_root)
    android_root = repo_root / "src-tauri" / "gen" / "android"
    app_dir = android_root / "app"

    tauri_props = app_dir / "tauri.properties"
    if tauri_props.is_file():
        lines = tauri_props.read_text(encoding="utf-8").splitlines()
        out_lines = []
        seen_code = False
        seen_name = False
        for line in lines:
            if line.startswith("tauri.android.versionCode="):
                out_lines.append(f"tauri.android.versionCode={version_code}")
                seen_code = True
            elif line.startswith("tauri.android.versionName="):
                out_lines.append(f"tauri.android.versionName={version_name}")
                seen_name = True
            else:
                out_lines.append(line)
        if not seen_code:
            out_lines.append(f"tauri.android.versionCode={version_code}")
        if not seen_name:
            out_lines.append(f"tauri.android.versionName={version_name}")
        tauri_props.write_text("\n".join(out_lines) + "\n", encoding="utf-8")
        print(f"Updated {tauri_props} (versionCode={version_code}, versionName={version_name})")

    build_gradle = app_dir / "build.gradle.kts"
    if build_gradle.is_file():
        text = build_gradle.read_text(encoding="utf-8")
        if re.search(r"\bcompileSdk\s*=\s*\d+", text):
            text = re.sub(r"\bcompileSdk\s*=\s*\d+", "compileSdk = 36", text)
        elif re.search(r"\bcompileSdkVersion\s*\(?\s*\d+\s*\)?", text):
            text = re.sub(r"\bcompileSdkVersion\s*\(?\s*\d+\s*\)?", "compileSdk = 36", text)
        else:
            text = re.sub(r"(\bandroid\s*\{)", r"\1\n    compileSdk = 36", text, count=1)

        if re.search(r"\btargetSdk\s*=\s*\d+", text):
            text = re.sub(r"\btargetSdk\s*=\s*\d+", "targetSdk = 36", text)
        elif re.search(r"\btargetSdkVersion\s*\(?\s*\d+\s*\)?", text):
            text = re.sub(r"\btargetSdkVersion\s*\(?\s*\d+\s*\)?", "targetSdk = 36", text)
        else:
            text = re.sub(r"(\bdefaultConfig\s*\{)", r"\1\n        targetSdk = 36", text, count=1)

        if not re.search(r"\bcompileSdk\s*=\s*36\b", text) or not re.search(r"\btargetSdk\s*=\s*36\b", text):
            raise RuntimeError(f"Could not enforce compileSdk/targetSdk 36 in {build_gradle}")
        text = re.sub(
            r'tauriProperties\.getProperty\(\s*"tauri\.android\.versionCode"\s*,\s*"[^"]*"\s*\)',
            f'tauriProperties.getProperty("tauri.android.versionCode", "{version_code}")',
            text,
        )
        if re.search(r"\bndkVersion\s*=", text):
            text = re.sub(r'\bndkVersion\s*=\s*"[^"]*"', 'ndkVersion = "28.2.13676358"', text)
        else:
            text = re.sub(
                r"(compileSdk\s*=\s*36\b[^\n]*\n)",
                r'\1    ndkVersion = "28.2.13676358"\n',
                text,
                count=1,
            )
        if "useLegacyPackaging" not in text:
            if re.search(r"\bpackaging\s*\{", text):
                text = re.sub(
                    r"(\bpackaging\s*\{)",
                    r"\1\n        jniLibs {\n            useLegacyPackaging = false\n        }",
                    text,
                    count=1,
                )
            else:
                text = re.sub(
                    r"(\bandroid\s*\{)",
                    r"\1\n    packaging {\n        jniLibs {\n            useLegacyPackaging = false\n        }\n    }",
                    text,
                    count=1,
                )
        build_gradle.write_text(text, encoding="utf-8")
        print(f"Updated {build_gradle} (compileSdk/targetSdk=36, ndkVersion=28.2.13676358)")

    gradle_props = android_root / "gradle.properties"
    if gradle_props.is_file():
        gp = gradle_props.read_text(encoding="utf-8")
        if "android.suppressUnsupportedCompileSdk=" not in gp:
            if gp and not gp.endswith("\n"):
                gp += "\n"
            gp += "android.suppressUnsupportedCompileSdk=36\n"
            gradle_props.write_text(gp, encoding="utf-8")
            print(f"Updated {gradle_props}")


def apply_overlay(repo_root: Path) -> None:
    overlay_root = repo_root / "src-tauri" / "android-overlay"
    overlay_java = overlay_root / "java" / "org" / "ciup" / "wifi"
    nsc_src = overlay_root / "res" / "xml" / "network_security_config.xml"
    android_main = repo_root / "src-tauri" / "gen" / "android" / "app" / "src" / "main"
    target_java = android_main / "java" / "org" / "ciup" / "wifi"
    target_xml = android_main / "res" / "xml"
    manifest_path = android_main / "AndroidManifest.xml"
    proguard_path = repo_root / "src-tauri" / "gen" / "android" / "app" / "proguard-rules.pro"

    if not overlay_java.is_dir():
        raise RuntimeError(f"Overlay directory missing: {overlay_java}")
    if not nsc_src.is_file():
        raise RuntimeError(f"Network security config missing: {nsc_src}")
    if not manifest_path.is_file():
        raise RuntimeError(f"Generated AndroidManifest.xml missing: {manifest_path}")

    target_xml.mkdir(parents=True, exist_ok=True)
    shutil.copy2(nsc_src, target_xml / nsc_src.name)
    print(f"Copied {nsc_src.name} -> {target_xml}")

    target_java.mkdir(parents=True, exist_ok=True)
    overlay_names = {p.name for p in overlay_java.glob("*.kt")}
    for existing in target_java.glob("*.kt"):
        if existing.name != "TauriActivity.kt" and existing.name not in overlay_names:
            existing.unlink()
            print(f"Removed stale {existing.name}")

    for src in sorted(overlay_java.glob("*.kt")):
        shutil.copy2(src, target_java / src.name)
        print(f"Copied {src.name} -> {target_java}")

    patch_manifest(manifest_path)
    print(f"Patched {manifest_path}")

    patch_proguard(proguard_path)
    print(f"Updated {proguard_path}")

    patch_gradle_config(repo_root)


if __name__ == "__main__":
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
    apply_overlay(root)
