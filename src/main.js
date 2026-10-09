// main.js — Clean, minimal Wi-Fi login logic
const invoke = window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke;

let checkInterval = null;

function setStatus(msg, type = 'info') {
    const el = document.getElementById('status-msg');
    el.textContent = msg;
    el.className   = `status-msg ${type}`;
    el.style.display = 'block';
}

function clearStatus() {
    document.getElementById('status-msg').style.display = 'none';
}

function showConnected(username) {
    document.getElementById('card-login').style.display  = 'none';
    document.getElementById('card-status').style.display = 'block';
    if (username) {
        document.getElementById('current-user').textContent = username;
    }
    const timeStr = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    document.getElementById('status-time').textContent = `Online (${timeStr})`;

    // Automatically register OS background task
    invoke('setup_background_task', { enable: true }).catch(() => {});
}

function showLogin() {
    document.getElementById('card-status').style.display = 'none';
    document.getElementById('card-login').style.display  = 'block';
}

async function handleConnect() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;

    if (!username || !password) {
        setStatus('Please enter username and password.', 'error');
        return;
    }

    const btn = document.getElementById('btn-login');
    btn.disabled    = true;
    btn.textContent = 'Checking…';
    setStatus('Checking network connection…', 'info');

    try {
        // 1. Always save credentials locally
        await invoke('save_credentials', { username, password });

        // 2. Check if internet is already working
        const status = await invoke('check_connectivity');
        if (status === 'connected') {
            setStatus('Credentials saved. Internet is already active!', 'info');
            btn.disabled    = false;
            btn.textContent = 'Connect';
            showConnected(username);
            return;
        }

        // 3. Internet is down / captive portal detected -> perform login
        btn.textContent = 'Connecting…';
        setStatus('Connecting to captive portal…', 'info');

        const success = await invoke('login_to_portal', { username, password });
        if (success) {
            clearStatus();
            showConnected(username);
        } else {
            setStatus('Login failed. Please check your username and password.', 'error');
        }
    } catch (err) {
        setStatus(`Error: ${err}`, 'error');
    } finally {
        btn.disabled    = false;
        btn.textContent = 'Connect';
    }
}

// Check connection and re-authenticate if connection was lost
async function checkAndAutoLogin() {
    try {
        const status = await invoke('check_connectivity');
        if (status === 'connected') {
            const timeStr = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
            const timeEl = document.getElementById('status-time');
            if (timeEl) timeEl.textContent = `Online (${timeStr})`;
            return;
        }

        // Connection lost or captive portal intercepted -> auto re-login
        const creds = await invoke('load_credentials').catch(() => null);
        if (creds?.username && creds?.password) {
            console.log('[CiupWifi] Connection lost. Attempting auto re-login...');
            const success = await invoke('login_to_portal', { username: creds.username, password: creds.password });
            if (success) {
                showConnected(creds.username);
            }
        }
    } catch (e) {
        console.warn('[CiupWifi] Connectivity check error:', e);
    }
}

async function init() {
    // 1. Load saved credentials
    const creds = await invoke('load_credentials').catch(() => ({ username: '', password: '' }));
    if (creds.username) document.getElementById('username').value = creds.username;
    if (creds.password) document.getElementById('password').value = creds.password;

    // 2. Check current connectivity
    try {
        const status = await invoke('check_connectivity');
        if (status === 'connected') {
            // Already connected: save state & show connected screen without re-login
            showConnected(creds.username);
        } else if (creds.username && creds.password) {
            // Captive portal detected: auto-login immediately
            await handleConnect();
        }
    } catch {
        // Fallback to login screen
    }

    // 3. Keep connection monitored every 60 seconds while window is open
    if (checkInterval) clearInterval(checkInterval);
    checkInterval = setInterval(checkAndAutoLogin, 60_000);

    // 4. Quietly check for new release updates in background
    setTimeout(checkForUpdates, 1500);
}

let pendingUpdate = null;

async function checkForUpdates() {
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

// Event Bindings
document.getElementById('btn-login').addEventListener('click', handleConnect);
document.getElementById('password').addEventListener('keydown', e => {
    if (e.key === 'Enter') handleConnect();
});
document.getElementById('btn-recheck').addEventListener('click', async () => {
    const btn = document.getElementById('btn-recheck');
    btn.disabled = true;
    btn.textContent = 'Checking…';
    await checkAndAutoLogin();
    btn.disabled = false;
    btn.textContent = 'Check Connection';
});
document.getElementById('btn-logout').addEventListener('click', () => {
    invoke('setup_background_task', { enable: false }).catch(() => {});
    showLogin();
    clearStatus();
});
document.getElementById('btn-update').addEventListener('click', async () => {
    if (!pendingUpdate) return;
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
        setStatus(`Update failed: ${e}`, 'error');
        btn.disabled = false;
        btn.textContent = 'Retry';
    }
});

init();
