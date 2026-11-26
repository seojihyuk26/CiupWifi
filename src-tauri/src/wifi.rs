use crate::state::AppState;

#[derive(Clone, Debug, Default)]
pub struct WifiInfo {
    pub is_wifi: bool,
    pub ssid: Option<String>,
    pub gateway_ip: Option<String>,
    pub domain_name: Option<String>,
    pub location_blocked: bool,
}

/// Returns true if the SSID belongs to CIUP WifiCity.
pub fn is_wificity_ssid(ssid: &str) -> bool {
    let clean = ssid.trim().trim_matches('"').to_lowercase();
    !clean.is_empty() && clean.contains("wificity")
}

/// Returns true if the gateway IP belongs to the CIUP 10.254.x.x subnet.
pub fn is_ciup_gateway(ip: &str) -> bool {
    ip.trim().starts_with("10.254.")
}

/// Returns true if the DHCP domain name belongs to CIUP / WifiCity.
pub fn is_ciup_domain(domain: &str) -> bool {
    let lower = domain.trim().to_lowercase();
    lower.contains("ciup") || lower.contains("wificity")
}

/// Evaluates whether the device is on WifiCity without performing network probes.
/// Returns `Some(true)` if definitively WifiCity, `Some(false)` if definitively not,
/// or `None` when Wi-Fi is active but the OS redacted the SSID and the gateway is not yet known.
pub fn is_known_wificity_without_probe(wifi_info: &WifiInfo, state: &AppState) -> Option<bool> {
    if let Some(ref ssid) = wifi_info.ssid {
        return Some(is_wificity_ssid(ssid));
    }
    if !wifi_info.is_wifi {
        return Some(false);
    }
    if let Some(ref gw) = wifi_info.gateway_ip {
        let clean = gw.trim();
        if is_ciup_gateway(clean) || state.known_ciup_gateways.iter().any(|k| k == clean) {
            return Some(true);
        }
    }
    if let Some(ref dom) = wifi_info.domain_name {
        if is_ciup_domain(dom) {
            return Some(true);
        }
    }
    None
}

/// Query current Wi-Fi status, SSID, and default gateway on the host OS.
pub fn get_current_wifi_info() -> WifiInfo {
    #[cfg(target_os = "windows")]
    {
        return get_windows_wifi_info();
    }
    #[cfg(target_os = "macos")]
    {
        return get_macos_wifi_info();
    }
    #[cfg(target_os = "linux")]
    {
        return get_linux_wifi_info();
    }
    #[cfg(target_os = "android")]
    {
        return WifiInfo::default();
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "android"
    )))]
    {
        WifiInfo::default()
    }
}

/// Disconnect from WifiCity on platforms that support clean Wi-Fi association drop.
pub fn disconnect_wifi() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        return disconnect_windows_wificity();
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn get_windows_wifi_info() -> WifiInfo {
    use windows_sys::Win32::Foundation::{ERROR_ACCESS_DENIED, ERROR_SUCCESS, HANDLE};
    use windows_sys::Win32::NetworkManagement::WiFi::{
        wlan_interface_state_connected, wlan_intf_opcode_current_connection, WlanCloseHandle,
        WlanEnumInterfaces, WlanFreeMemory, WlanOpenHandle, WlanQueryInterface,
        WLAN_CONNECTION_ATTRIBUTES, WLAN_INTERFACE_INFO_LIST,
    };

    let mut info = WifiInfo::default();

    unsafe {
        let mut negotiated: u32 = 0;
        let mut client_handle: HANDLE = 0 as HANDLE;
        if WlanOpenHandle(2, std::ptr::null(), &mut negotiated, &mut client_handle) == ERROR_SUCCESS {
            let mut iface_list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
            if WlanEnumInterfaces(client_handle, std::ptr::null(), &mut iface_list_ptr) == ERROR_SUCCESS
                && !iface_list_ptr.is_null()
            {
                let count = (*iface_list_ptr).dwNumberOfItems as usize;
                let ifaces = std::slice::from_raw_parts(
                    (*iface_list_ptr).InterfaceInfo.as_ptr(),
                    count,
                );

                for iface in ifaces {
                    if iface.isState == wlan_interface_state_connected {
                        info.is_wifi = true;
                        let mut data_size: u32 = 0;
                        let mut data_ptr = std::ptr::null_mut();
                        let query_res = WlanQueryInterface(
                            client_handle,
                            &iface.InterfaceGuid,
                            wlan_intf_opcode_current_connection,
                            std::ptr::null(),
                            &mut data_size,
                            &mut data_ptr,
                            std::ptr::null_mut(),
                        );

                        if query_res == ERROR_SUCCESS && !data_ptr.is_null() {
                            let conn = &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                            let len = (conn.wlanAssociationAttributes.dot11Ssid.uSSIDLength as usize).min(32);
                            let bytes = &conn.wlanAssociationAttributes.dot11Ssid.ucSSID[..len];
                            let ssid = String::from_utf8_lossy(bytes).trim().to_string();
                            WlanFreeMemory(data_ptr as *const std::ffi::c_void);
                            if !ssid.is_empty() {
                                info.ssid = Some(ssid);
                                break;
                            }
                        } else if query_res == ERROR_ACCESS_DENIED {
                            info.location_blocked = true;
                        }
                    }
                }
                WlanFreeMemory(iface_list_ptr as *const std::ffi::c_void);
            }
            WlanCloseHandle(client_handle, std::ptr::null());
        }
    }

    if info.ssid.is_none() {
        let netsh_info = parse_netsh_wlan();
        if netsh_info.is_wifi {
            info.is_wifi = true;
        }
        if netsh_info.ssid.is_some() {
            info.ssid = netsh_info.ssid;
            info.location_blocked = false;
        } else if netsh_info.location_blocked {
            info.location_blocked = true;
        }
    }

    if info.is_wifi {
        info.gateway_ip = get_windows_gateway();
    }

    info
}

#[cfg(target_os = "windows")]
fn parse_netsh_wlan() -> WifiInfo {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut info = WifiInfo::default();
    let output = match std::process::Command::new("netsh")
        .args(["wlan", "show", "interfaces"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
    {
        Ok(o) => o,
        Err(_) => return info,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lower = stdout.to_lowercase();
    if lower.contains("location") || lower.contains("위치") {
        info.location_blocked = true;
    }

    for line in stdout.lines() {
        let trimmed = line.trim();
        let trimmed_lower = trimmed.to_lowercase();

        if trimmed_lower.starts_with("state") || trimmed_lower.starts_with("상태") || trimmed_lower.starts_with("état") {
            if let Some((_, val)) = trimmed.split_once(':') {
                let v = val.trim().to_lowercase();
                if v == "connected" || v == "연결됨" || v == "connecté" {
                    info.is_wifi = true;
                }
            }
        }

        if let Some((key, val)) = trimmed.split_once(':') {
            let k = key.trim().to_lowercase();
            if k == "ssid" {
                let ssid = val.trim().to_string();
                if !ssid.is_empty() {
                    info.is_wifi = true;
                    info.ssid = Some(ssid);
                    break;
                }
            }
        }
    }

    info
}

#[cfg(target_os = "windows")]
fn get_windows_gateway() -> Option<String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = std::process::Command::new("route")
        .args(["print", "0.0.0.0"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[0] == "0.0.0.0" && parts[1] == "0.0.0.0" {
            let gw = parts[2];
            if gw.split('.').count() == 4 {
                return Some(gw.to_string());
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn disconnect_windows_wificity() -> Result<(), String> {
    use windows_sys::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
    use windows_sys::Win32::NetworkManagement::WiFi::{
        wlan_interface_state_connected, wlan_intf_opcode_current_connection, WlanCloseHandle,
        WlanDisconnect, WlanEnumInterfaces, WlanFreeMemory, WlanOpenHandle, WlanQueryInterface,
        WLAN_CONNECTION_ATTRIBUTES, WLAN_INTERFACE_INFO_LIST,
    };

    unsafe {
        let mut negotiated: u32 = 0;
        let mut client_handle: HANDLE = 0 as HANDLE;
        if WlanOpenHandle(2, std::ptr::null(), &mut negotiated, &mut client_handle) != ERROR_SUCCESS {
            return Ok(());
        }

        let mut iface_list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
        if WlanEnumInterfaces(client_handle, std::ptr::null(), &mut iface_list_ptr) == ERROR_SUCCESS
            && !iface_list_ptr.is_null()
        {
            let count = (*iface_list_ptr).dwNumberOfItems as usize;
            let ifaces = std::slice::from_raw_parts(
                (*iface_list_ptr).InterfaceInfo.as_ptr(),
                count,
            );

            for iface in ifaces {
                if iface.isState != wlan_interface_state_connected {
                    continue;
                }
                let mut data_size: u32 = 0;
                let mut data_ptr = std::ptr::null_mut();
                let mut should_disconnect = false;

                if WlanQueryInterface(
                    client_handle,
                    &iface.InterfaceGuid,
                    wlan_intf_opcode_current_connection,
                    std::ptr::null(),
                    &mut data_size,
                    &mut data_ptr,
                    std::ptr::null_mut(),
                ) == ERROR_SUCCESS
                    && !data_ptr.is_null()
                {
                    let conn = &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                    let len = (conn.wlanAssociationAttributes.dot11Ssid.uSSIDLength as usize).min(32);
                    let bytes = &conn.wlanAssociationAttributes.dot11Ssid.ucSSID[..len];
                    let ssid = String::from_utf8_lossy(bytes);
                    if is_wificity_ssid(&ssid) {
                        should_disconnect = true;
                    }
                    WlanFreeMemory(data_ptr as *const std::ffi::c_void);
                } else if let Some(gw) = get_windows_gateway() {
                    if is_ciup_gateway(&gw) {
                        should_disconnect = true;
                    }
                }

                if should_disconnect {
                    WlanDisconnect(client_handle, &iface.InterfaceGuid, std::ptr::null());
                }
            }
            WlanFreeMemory(iface_list_ptr as *const std::ffi::c_void);
        }

        WlanCloseHandle(client_handle, std::ptr::null());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_macos_wifi_device() -> String {
    if let Ok(output) = std::process::Command::new("/usr/sbin/networksetup")
        .args(["-listallhardwareports"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut found_wifi = false;
        for line in stdout.lines() {
            let t = line.trim();
            if t.starts_with("Hardware Port:") {
                let lower = t.to_lowercase();
                found_wifi = lower.contains("wi-fi") || lower.contains("airport");
            } else if found_wifi && t.starts_with("Device:") {
                if let Some((_, dev)) = t.split_once(':') {
                    let dev = dev.trim();
                    if !dev.is_empty() {
                        return dev.to_string();
                    }
                }
                found_wifi = false;
            }
        }
    }
    "en0".to_string()
}

#[cfg(target_os = "macos")]
fn get_macos_wifi_info() -> WifiInfo {
    let dev = get_macos_wifi_device();
    let mut info = WifiInfo::default();

    // 1. Check link status & non-link-local IPv4 via /sbin/ifconfig (immune to Sequoia/Tahoe SSID redaction)
    if let Ok(output) = std::process::Command::new("/sbin/ifconfig")
        .arg(&dev)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let t = line.trim();
            if t == "status: active" || t.starts_with("status: active") {
                info.is_wifi = true;
            } else if let Some(rest) = t.strip_prefix("inet ") {
                let ip = rest.split_whitespace().next().unwrap_or("");
                if !ip.is_empty() && !ip.starts_with("127.") && !ip.starts_with("169.254.") {
                    info.is_wifi = true;
                }
            }
        }
    }

    // 2. Inspect ipconfig getsummary <dev> for LinkStatusActive and SSID
    if let Ok(output) = std::process::Command::new("/usr/sbin/ipconfig")
        .args(["getsummary", &dev])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let t = line.trim();
            if t.starts_with("LinkStatusActive") && t.contains("TRUE") {
                info.is_wifi = true;
            } else if t.starts_with("SSID :") || t.starts_with("SSID:") {
                if let Some((_, val)) = t.split_once(':') {
                    let ssid = val.trim().to_string();
                    if !ssid.is_empty() {
                        info.is_wifi = true;
                        if ssid == "<redacted>" {
                            info.location_blocked = true;
                        } else {
                            info.ssid = Some(ssid);
                            info.location_blocked = false;
                        }
                    }
                }
            }
        }
    }

    // 3. Only try networksetup if Wi-Fi is active and SSID still missing; NEVER early-return on "not associated"!
    if info.is_wifi && info.ssid.is_none() {
        if let Ok(output) = std::process::Command::new("/usr/sbin/networksetup")
            .args(["-getairportnetwork", &dev])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lower = stdout.to_lowercase();
            if lower.contains("current wi-fi network:") || lower.contains("current airport network:") {
                if let Some((_, val)) = stdout.split_once(':') {
                    let ssid = val.trim().to_string();
                    if !ssid.is_empty() && ssid != "<redacted>" {
                        info.ssid = Some(ssid);
                        info.location_blocked = false;
                    }
                }
            }
        }
    }

    // 4. Query DHCP router / default gateway + domain_name whenever Wi-Fi is active
    if info.is_wifi {
        if let Ok(output) = std::process::Command::new("/usr/sbin/ipconfig")
            .args(["getoption", &dev, "router"])
            .output()
        {
            let gw = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !gw.is_empty() {
                info.gateway_ip = Some(gw);
            }
        }

        if info.gateway_ip.is_none() {
            if let Ok(output) = std::process::Command::new("/sbin/route")
                .args(["-n", "get", "default"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let t = line.trim();
                    if let Some(rest) = t.strip_prefix("gateway:") {
                        let gw = rest.trim();
                        if !gw.is_empty() {
                            info.gateway_ip = Some(gw.to_string());
                            break;
                        }
                    }
                }
            }
        }

        if let Ok(output) = std::process::Command::new("/usr/sbin/ipconfig")
            .args(["getoption", &dev, "domain_name"])
            .output()
        {
            let dom = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !dom.is_empty() {
                info.domain_name = Some(dom);
            }
        }

        if info.ssid.is_none() {
            info.location_blocked = true;
        }
    }

    info
}

#[cfg(target_os = "linux")]
fn get_linux_wifi_info() -> WifiInfo {
    let mut info = WifiInfo::default();

    if let Ok(output) = std::process::Command::new("nmcli")
        .args(["-t", "-f", "active,ssid", "dev", "wifi"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(ssid) = line.strip_prefix("yes:") {
                    let clean = ssid.trim().to_string();
                    if !clean.is_empty() {
                        info.is_wifi = true;
                        info.ssid = Some(clean);
                        break;
                    }
                }
            }
        }
    }

    if info.ssid.is_none() {
        if let Ok(output) = std::process::Command::new("iwgetid").arg("-r").output() {
            if output.status.success() {
                let ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !ssid.is_empty() {
                    info.is_wifi = true;
                    info.ssid = Some(ssid);
                }
            }
        }
    }

    if info.is_wifi {
        if let Ok(output) = std::process::Command::new("ip")
            .args(["route", "show", "default"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.split_whitespace().collect();
            if let Some(pos) = parts.iter().position(|&p| p == "via") {
                if let Some(gw) = parts.get(pos + 1) {
                    info.gateway_ip = Some((*gw).to_string());
                }
            }
        }
    }

    info
}
