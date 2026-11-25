// main.js — Unified frontend across Desktop (Tauri) and Android (Native WifiMonitorService)
const invoke = window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke;

const hasAndroidBridge = () => typeof window.AndroidWifi !== 'undefined';

const pendingAndroidRequests = new Map();
let reqCounter = 0;

window.__onAndroidAsyncResult = (requestId, payload) => {
    const entry = pendingAndroidRequests.get(requestId);
    if (!entry) return;
    clearTimeout(entry.timer);
    pendingAndroidRequests.delete(requestId);
    entry.resolve(payload);
};

function callAndroidAsync(methodName, ...args) {
    return new Promise((resolve, reject) => {
        const id = `req_${++reqCounter}_${Date.now()}`;
        const timer = setTimeout(() => {
            pendingAndroidRequests.delete(id);
            reject(new Error('Operation timed out'));
        }, 20_000);
        pendingAndroidRequests.set(id, { resolve, reject, timer });
        try {
            window.AndroidWifi[methodName](id, ...args);
        } catch (err) {
            clearTimeout(timer);
            pendingAndroidRequests.delete(id);
            reject(err);
        }
    });
}

const Backend = {
    async getNetworkStatus() {
        if (hasAndroidBridge()) {
            return JSON.parse(window.AndroidWifi.getNetworkStatus());
        }
        return invoke('get_network_status');
    },
    async loadCredentials() {
        if (hasAndroidBridge()) {
            return JSON.parse(window.AndroidWifi.loadCredentials());
        }
        return invoke('load_credentials');
    },
    async saveCredentials(username, password) {
        if (hasAndroidBridge()) {
            window.AndroidWifi.saveCredentials(username, password);
            return;
        }
        return invoke('save_credentials', { username, password });
    },
    async connectNow(username, password) {
        if (hasAndroidBridge()) {
            return callAndroidAsync('connectNow');
        }
        return invoke('login_to_portal', { username, password });
    },
    async disconnectWifiCity() {
        if (hasAndroidBridge()) {
            return callAndroidAsync('disconnectNow');
        }
        return invoke('disconnect_wificity');
    },
    async setBackgroundAuto(enable) {
        if (hasAndroidBridge()) {
            window.AndroidWifi.setAutoLogin(Boolean(enable));
            return;
        }
        return invoke('setup_background_task', { enable });
    },
    getAndroidPermissions() {
        if (!hasAndroidBridge()) return null;
        try {
            return JSON.parse(window.AndroidWifi.getPermissionsStatus());
        } catch {
            return null;
        }
    },
    requestAndroidPermissions() {
        if (hasAndroidBridge()) window.AndroidWifi.requestPermissions();
    },
    openAndroidWifiSettings() {
        if (hasAndroidBridge()) window.AndroidWifi.openWifiSettings();
    },
    requestIgnoreBattery() {
        if (hasAndroidBridge()) window.AndroidWifi.requestIgnoreBatteryOptimizations();
    },
};

let statusInterval = null;
let editingAccount = false;
let lastCreds = { username: '', password: '' };
let lastStatus = null;
let pendingUpdate = null;

function setLoginStatus(msg, type = 'info') {
    const el = document.getElementById('status-msg');
    el.textContent = msg;
    el.className = `status-msg ${type}`;
    el.style.display = 'block';
}

function clearLoginStatus() {
    document.getElementById('status-msg').style.display = 'none';
}

function setCardStatusMsg(msg, type = 'info') {
    const el = document.getElementById('card-status-msg');
    if (!msg) {
        el.style.display = 'none';
        return;
    }
    el.textContent = msg;
    el.className = `status-msg ${type}`;
    el.style.display = 'block';
}

function formatPortalLabel(portalUrl) {
    if (!portalUrl) return null;
    try {
        const u = new URL(portalUrl);
        return `${u.protocol.replace(':', '').toUpperCase()} • ${u.host}`;
    } catch {
        return portalUrl;
    }
}

function updateAndroidPermBanner() {
    const banner = document.getElementById('perm-banner');
    if (!hasAndroidBridge()) {
        banner.style.display = 'none';
        return;
    }
    const perms = Backend.getAndroidPermissions();
    if (!perms) {
        banner.style.display = 'none';
        return;
    }

    const needLoc = !perms.locationGranted;
    const needNotif = !perms.notificationsGranted;
    const needBattery = !perms.batteryIgnored;

    if (!needLoc && !needNotif && !needBattery) {
        banner.style.display = 'none';
        return;
    }

    const textEl = document.getElementById('perm-text');
    const grantBtn = document.getElementById('btn-grant-perm');
    const batBtn = document.getElementById('btn-battery-perm');

    if (needLoc || needNotif) {
        textEl.textContent = needLoc
            ? 'Grant Location access so CiupWifi can detect WifiCity automatically.'
            : 'Allow notifications so Android keeps auto-login active in the background.';
        grantBtn.style.display = 'inline-block';
    } else if (!perms.locationEnabled) {
        textEl.textContent = 'Turn on device Location (GPS) so Android exposes the Wi-Fi name.';
        grantBtn.style.display = 'none';
    } else {
        textEl.textContent = 'Disable battery optimization for reliable background Wi-Fi renewal.';
        grantBtn.style.display = 'none';
    }

    batBtn.style.display = needBattery ? 'inline-block' : 'none';
    banner.style.display = 'flex';
}

function showLoginView(showBackButton = false) {
    document.getElementById('card-status').style.display = 'none';
    document.getElementById('card-login').style.display = 'block';
    document.getElementById('btn-cancel-edit').style.display = showBackButton ? 'block' : 'none';

    const sub = document.getElementById('login-sub');
    if (lastStatus) {
        if (lastStatus.state === 'not_on_wificity') {
            sub.textContent = lastStatus.detail || 'Switch to WifiCity to authenticate';
        } else if (lastStatus.state === 'disconnected') {
            sub.textContent = 'Wi-Fi is disconnected — credentials will auto-run on WifiCity';
        } else if (lastStatus.state === 'wifi_city_captive') {
            sub.textContent = 'Connected to WifiCity • Portal ready';
        } else {
            sub.textContent = 'Auto-login for CIUP Campus Network';
        }
    }
}

function renderStatus(status) {
    lastStatus = status;
    updateAndroidPermBanner();

    const hasCreds = Boolean(lastCreds.username && lastCreds.password);

    // If user is explicitly editing credentials or has never saved any yet, stay on login card
    if (editingAccount || !hasCreds) {
        showLoginView(hasCreds);
        if (status.auth_failed && status.detail) {
            setLoginStatus(status.detail, 'error');
        }
        return;
    }

    // If auth failed on WifiCity with wrong password, surface the login card with the error
    if (status.state === 'wifi_city_captive' && status.auth_failed) {
        showLoginView(false);
        setLoginStatus(status.detail || 'Login rejected. Please verify username and password.', 'error');
        return;
    }

    document.getElementById('card-login').style.display = 'none';
    document.getElementById('card-status').style.display = 'block';

    const dot = document.getElementById('status-dot');
    const ring = document.getElementById('pulse-ring');
    const title = document.getElementById('status-title');
    const sub = document.getElementById('status-sub');
    const netEl = document.getElementById('status-network');
    const ssidEl = document.getElementById('status-ssid');
    const userEl = document.getElementById('current-user');
    const portalRow = document.getElementById('row-portal');
    const portalEl = document.getElementById('status-portal');
    const autoEl = document.getElementById('status-auto');
    const hintEl = document.getElementById('status-hint');

    const primaryBtn = document.getElementById('btn-primary-action');
    const disconnectBtn = document.getElementById('btn-disconnect');
    const wifiSettingsBtn = document.getElementById('btn-wifi-settings');

    userEl.textContent = lastCreds.username || '—';
    const portalLabel = formatPortalLabel(status.portal_url);
    if (portalLabel) {
        portalRow.style.display = 'flex';
        portalEl.textContent = portalLabel;
    } else {
        portalRow.style.display = 'none';
    }

    wifiSettingsBtn.style.display = hasAndroidBridge() ? 'block' : 'none';
    setCardStatusMsg(null);

    const timeStr = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });

    dot.className = 'status-dot';
    ring.className = 'pulse-ring';

    if (status.state === 'wifi_city_online') {
        dot.classList.add('online');
        ring.style.display = 'block';
        title.textContent = 'Connected';
        sub.textContent = 'Cité Universitaire • WifiCity';
        netEl.textContent = `Online (${timeStr})`;
        netEl.className = 'value online';
        ssidEl.textContent = status.ssid || 'WifiCity';
        autoEl.textContent = '⚡ Active (Auto-Reauth)';
        autoEl.className = 'value text-emerald';
        hintEl.textContent = 'WifiCity is online. If the connection drops, CiupWifi re-authenticates automatically.';
        primaryBtn.textContent = 'Check Connection';
        primaryBtn.dataset.action = 'check';
        disconnectBtn.style.display = 'block';
    } else if (status.state === 'wifi_city_captive') {
        dot.classList.add('connecting');
        ring.classList.add('amber');
        ring.style.display = 'block';
        title.textContent = 'Portal Detected';
        sub.textContent = 'WifiCity requires login';
        netEl.textContent = 'Captive Portal';
        netEl.className = 'value warn';
        ssidEl.textContent = status.ssid || 'WifiCity';
        autoEl.textContent = '⚡ Connecting…';
        autoEl.className = 'value text-amber';
        hintEl.textContent = 'Authenticating with the WifiCity portal automatically.';
        if (status.detail) setCardStatusMsg(status.detail, 'info');
        primaryBtn.textContent = 'Connect Now';
        primaryBtn.dataset.action = 'connect';
        disconnectBtn.style.display = 'block';
    } else if (status.state === 'paused') {
        dot.classList.add('paused');
        ring.style.display = 'none';
        title.textContent = 'Disconnected';
        sub.textContent = 'WifiCity auto-reconnect paused';
        netEl.textContent = 'Paused by user';
        netEl.className = 'value warn';
        ssidEl.textContent = status.ssid || 'WifiCity';
        autoEl.textContent = '⏸ Paused until Connect';
        autoEl.className = 'value text-amber';
        hintEl.textContent = 'Stays disconnected until you tap Connect or rejoin WifiCity.';
        primaryBtn.textContent = 'Connect';
        primaryBtn.dataset.action = 'connect';
        disconnectBtn.style.display = 'none';
    } else if (status.state === 'permission_needed') {
        dot.classList.add('paused');
        ring.style.display = 'none';
        title.textContent = 'Permission Needed';
        sub.textContent = 'Cannot read current Wi-Fi name';
        netEl.textContent = 'Wi-Fi (SSID hidden)';
        netEl.className = 'value warn';
        ssidEl.textContent = 'Hidden by OS';
        autoEl.textContent = 'Waiting for permission';
        autoEl.className = 'value text-amber';
        hintEl.textContent = status.detail || 'Enable Location access so CiupWifi can verify WifiCity.';
        primaryBtn.textContent = hasAndroidBridge() ? 'Grant Permission' : 'Refresh Status';
        primaryBtn.dataset.action = hasAndroidBridge() ? 'permission' : 'check';
        disconnectBtn.style.display = 'none';
    } else if (status.state === 'not_on_wificity') {
        dot.classList.add('idle');
        ring.style.display = 'none';
        title.textContent = status.is_cellular ? 'On Mobile Data' : 'Not on WifiCity';
        sub.textContent = status.detail || 'Connect to WifiCity to use auto-login';
        netEl.textContent = status.is_cellular ? 'Cellular (4G/5G)' : 'Other Wi-Fi';
        netEl.className = 'value muted';
        ssidEl.textContent = status.is_cellular ? 'Wi-Fi Off / Inactive' : (status.ssid || 'Unknown');
        autoEl.textContent = '🛡 Armed for WifiCity';
        autoEl.className = 'value text-emerald';
        hintEl.textContent = 'Join the "WifiCity" Wi-Fi network — CiupWifi will log in automatically.';
        primaryBtn.textContent = 'Refresh Status';
        primaryBtn.dataset.action = 'check';
        disconnectBtn.style.display = 'none';
    } else {
        dot.classList.add('idle');
        ring.style.display = 'none';
        title.textContent = 'Wi-Fi Disconnected';
        sub.textContent = 'Waiting for WifiCity network';
        netEl.textContent = 'Offline';
        netEl.className = 'value muted';
        ssidEl.textContent = 'None';
        autoEl.textContent = '🛡 Armed for WifiCity';
        autoEl.className = 'value text-emerald';
        hintEl.textContent = 'Connect to "WifiCity" in your Wi-Fi settings and CiupWifi will handle login automatically.';
        primaryBtn.textContent = 'Refresh Status';
        primaryBtn.dataset.action = 'check';
        disconnectBtn.style.display = 'none';
    }
}

async function refreshStatus() {
    try {
        const status = await Backend.getNetworkStatus();
        renderStatus(status);
        return status;
    } catch (e) {
        console.warn('[CiupWifi] Status check failed:', e);
        return null;
    }
}

async function handleSaveAndConnect() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;

    if (!username || !password) {
        setLoginStatus('Please enter username and password.', 'error');
        return;
    }

    const btn = document.getElementById('btn-login');
    btn.disabled = true;
    btn.textContent = 'Checking…';
    setLoginStatus('Saving credentials & checking Wi-Fi…', 'info');

    try {
        await Backend.saveCredentials(username, password);
        lastCreds = { username, password };
        await Backend.setBackgroundAuto(true).catch(() => {});

        const status = await Backend.getNetworkStatus();
        lastStatus = status;

        if (!status.is_wificity) {
            editingAccount = false;
            clearLoginStatus();
            renderStatus(status);
            return;
        }

        if (status.state === 'wifi_city_online') {
            editingAccount = false;
            clearLoginStatus();
            renderStatus(status);
            return;
        }

        btn.textContent = 'Connecting…';
        setLoginStatus('Connecting to WifiCity portal…', 'info');

        const res = await Backend.connectNow(username, password);
        if (res?.ok) {
            editingAccount = false;
            clearLoginStatus();
            await refreshStatus();
        } else {
            setLoginStatus(res?.message || 'Login failed. Please check credentials.', 'error');
        }
    } catch (err) {
        setLoginStatus(`Error: ${err}`, 'error');
    } finally {
        btn.disabled = false;
        btn.textContent = 'Save & Connect';
    }
}

async function handlePrimaryAction() {
    const btn = document.getElementById('btn-primary-action');
    const action = btn.dataset.action || 'check';

    if (action === 'permission') {
        Backend.requestAndroidPermissions();
        return;
    }

    btn.disabled = true;
    const origText = btn.textContent;
    btn.textContent = action === 'connect' ? 'Connecting…' : 'Checking…';

    try {
        if (action === 'connect' && lastCreds.username && lastCreds.password) {
            const res = await Backend.connectNow(lastCreds.username, lastCreds.password);
            if (!res?.ok && res?.message) {
                setCardStatusMsg(res.message, res.auth_failed ? 'error' : 'info');
            }
        }
        await refreshStatus();
    } catch (e) {
        setCardStatusMsg(`Error: ${e}`, 'error');
    } finally {
        btn.disabled = false;
        if (btn.textContent === 'Connecting…' || btn.textContent === 'Checking…') {
            btn.textContent = origText;
        }
    }
}

async function handleDisconnect() {
    const btn = document.getElementById('btn-disconnect');
    btn.disabled = true;
    btn.textContent = 'Disconnecting…';
    try {
        await Backend.disconnectWifiCity();
        await refreshStatus();
    } catch (e) {
        setCardStatusMsg(`Disconnect error: ${e}`, 'error');
    } finally {
        btn.disabled = false;
        btn.textContent = 'Disconnect WifiCity';
    }
}

async function checkForUpdates() {
    if (!invoke) return;
    try {
        const update = await invoke('check_for_updates');
        if (update) {
            pendingUpdate = update;
            const banner = document.getElementById('update-banner');
            const text = document.getElementById('update-text');
            if (banner && text) {
                text.textContent = `✨ v${update.latest_version} available!`;
                banner.style.display = 'flex';
            }
        }
    } catch (e) {
        console.log('[CiupWifi] Update check:', e);
    }
}

async function init() {
    window.__onAndroidWifiStatus = (status) => {
        if (status) renderStatus(status);
    };
    window.__onAndroidPermissionsChanged = () => {
        updateAndroidPermBanner();
        refreshStatus();
    };

    const creds = await Backend.loadCredentials().catch(() => ({ username: '', password: '' }));
    lastCreds = {
        username: creds?.username || '',
        password: creds?.password || '',
    };
    if (lastCreds.username) document.getElementById('username').value = lastCreds.username;
    if (lastCreds.password) document.getElementById('password').value = lastCreds.password;

    if (lastCreds.username && lastCreds.password) {
        Backend.setBackgroundAuto(true).catch(() => {});
    }

    const status = await refreshStatus();

    // If already on WifiCity with captive portal and saved creds (and not paused / auth_failed), connect automatically
    if (
        status &&
        status.state === 'wifi_city_captive' &&
        !status.auth_failed &&
        !status.user_paused &&
        lastCreds.username &&
        lastCreds.password
    ) {
        Backend.connectNow(lastCreds.username, lastCreds.password)
            .then(() => refreshStatus())
            .catch(() => {});
    }

    if (statusInterval) clearInterval(statusInterval);
    statusInterval = setInterval(refreshStatus, 4000);

    setTimeout(checkForUpdates, 1500);
}

// Event bindings
document.getElementById('btn-login').addEventListener('click', handleSaveAndConnect);
document.getElementById('password').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') handleSaveAndConnect();
});
document.getElementById('btn-cancel-edit').addEventListener('click', () => {
    editingAccount = false;
    clearLoginStatus();
    refreshStatus();
});
document.getElementById('btn-primary-action').addEventListener('click', handlePrimaryAction);
document.getElementById('btn-disconnect').addEventListener('click', handleDisconnect);
document.getElementById('btn-wifi-settings').addEventListener('click', () => Backend.openAndroidWifiSettings());
document.getElementById('btn-logout').addEventListener('click', () => {
    editingAccount = true;
    clearLoginStatus();
    showLoginView(Boolean(lastCreds.username && lastCreds.password));
});
document.getElementById('btn-grant-perm').addEventListener('click', () => Backend.requestAndroidPermissions());
document.getElementById('btn-battery-perm').addEventListener('click', () => Backend.requestIgnoreBattery());

document.getElementById('btn-update').addEventListener('click', async () => {
    if (!pendingUpdate || !invoke) return;
    const btn = document.getElementById('btn-update');
    btn.disabled = true;
    btn.textContent = 'Updating…';

    try {
        const isAndroid = /Android/i.test(navigator.userAgent);
        if (isAndroid) {
            window.location.href = pendingUpdate.download_url;
            btn.textContent = 'Downloading…';
            return;
        }

        await invoke('download_and_install_update', {
            downloadUrl: pendingUpdate.download_url,
            assetName: pendingUpdate.asset_name,
        });
    } catch (e) {
        setLoginStatus(`Update failed: ${e}`, 'error');
        btn.disabled = false;
        btn.textContent = 'Retry';
    }
});

init();
