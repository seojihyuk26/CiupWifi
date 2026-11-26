#!/usr/bin/env python3
"""Applies the zero-permission event-driven Android overlay to Tauri's generated Android project."""

from pathlib import Path
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


if __name__ == "__main__":
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
    apply_overlay(root)
