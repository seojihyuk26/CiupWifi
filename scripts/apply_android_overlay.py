#!/usr/bin/env python3
"""Applies the native Android Wi-Fi monitor overlay to Tauri's generated Android project."""

from pathlib import Path
import shutil
import sys
import xml.etree.ElementTree as ET

ANDROID_NS = "http://schemas.android.com/apk/res/android"
ET.register_namespace("android", ANDROID_NS)

REQUIRED_PERMISSIONS = [
    "android.permission.INTERNET",
    "android.permission.ACCESS_NETWORK_STATE",
    "android.permission.CHANGE_NETWORK_STATE",
    "android.permission.ACCESS_WIFI_STATE",
    "android.permission.ACCESS_FINE_LOCATION",
    "android.permission.ACCESS_COARSE_LOCATION",
    "android.permission.FOREGROUND_SERVICE",
    "android.permission.FOREGROUND_SERVICE_SPECIAL_USE",
    "android.permission.POST_NOTIFICATIONS",
    "android.permission.RECEIVE_BOOT_COMPLETED",
    "android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS",
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

    # Service: .WifiMonitorService
    service_name = ".WifiMonitorService"
    has_service = any(
        el.get(android_attr("name")) in (service_name, "org.ciup.wifi.WifiMonitorService")
        for el in app_el.findall("service")
    )
    if not has_service:
        service_el = ET.SubElement(
            app_el,
            "service",
            {
                android_attr("name"): service_name,
                android_attr("enabled"): "true",
                android_attr("exported"): "false",
                android_attr("foregroundServiceType"): "specialUse",
            },
        )
        ET.SubElement(
            service_el,
            "property",
            {
                android_attr("name"): "android.app.PROPERTY_SPECIAL_USE_FGS_SUBTYPE",
                android_attr("value"): "Monitors WifiCity Wi-Fi connection and handles captive portal authentication automatically",
            },
        )

    # Receiver: .BootReceiver
    receiver_name = ".BootReceiver"
    has_receiver = any(
        el.get(android_attr("name")) in (receiver_name, "org.ciup.wifi.BootReceiver")
        for el in app_el.findall("receiver")
    )
    if not has_receiver:
        receiver_el = ET.SubElement(
            app_el,
            "receiver",
            {
                android_attr("name"): receiver_name,
                android_attr("enabled"): "true",
                android_attr("exported"): "true",
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


def apply_overlay(repo_root: Path) -> None:
    overlay_java = repo_root / "src-tauri" / "android-overlay" / "java" / "org" / "ciup" / "wifi"
    android_main = repo_root / "src-tauri" / "gen" / "android" / "app" / "src" / "main"
    target_java = android_main / "java" / "org" / "ciup" / "wifi"
    manifest_path = android_main / "AndroidManifest.xml"
    proguard_path = repo_root / "src-tauri" / "gen" / "android" / "app" / "proguard-rules.pro"

    if not overlay_java.is_dir():
        raise RuntimeError(f"Overlay directory missing: {overlay_java}")
    if not manifest_path.is_file():
        raise RuntimeError(f"Generated AndroidManifest.xml missing: {manifest_path}")

    target_java.mkdir(parents=True, exist_ok=True)
    for src in sorted(overlay_java.glob("*.kt")):
        shutil.copy2(src, target_java / src.name)
        print(f"Copied {src.name} -> {target_java}")

    patch_manifest(manifest_path)
    print(f"Patched {manifest_path}")

    patch_proguard(proguard_path)
    print(f"Updated {proguard_path}")


if __name__ == "__main__":
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
    apply_overlay(root)
