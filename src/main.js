// main.js — wifiLogin.js logic ported to Tauri invoke API
const invoke = window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke;

// ── Config ────────────────────────────────────────────────────────────────────
const CONFIG = {
    FAILED_RETRY_THRESHOLD_MS: 15_000,
    PRE_EXPIRY_BUFFER_MS: 5 * 60_000,
    SESSION_HISTORY_MAX: 3,                 // Keep up to 3 recent records
    MIN_VALID_SESSION_MS: 3_600_000,        // Only record sessions lasting >= 1 hour
    MAX_VALID_SESSION_MS: 48 * 3_600_000,
};

// ── In-Memory State ───────────────────────────────────────────────────────────
let sessionStartTime = 0;
let sessionHistory   = [];   // Array of session durations in ms
let reloginTimer     = null;
let lastAttemptTime  = 0;
let wasProactive     = false;

// ── Helpers ───────────────────────────────────────────────────────────────────
function fmtDuration(ms) {
    const m = Math.round(ms / 60_000);
    return m >= 60 ? `${(m / 60).toFixed(1)} h` : `${m} min`;
}

function setStatus(msg, type = 'info') {
    const el = document.getElementById('status-msg');
    el.textContent = msg;
    el.className   = `status-msg ${type}`;
    el.style.display = 'block';
}

function clearStatus() {
    document.getElementById('status-msg').style.display = 'none';
}

// ── Session History ───────────────────────────────────────────────────────────
async function loadHistory() {
    try {
        sessionHistory = await invoke('load_session_history');
    } catch {
        sessionHistory = [];
    }
}

async function recordSessionEnd() {
    if (wasProactive || !sessionStartTime) return;
    wasProactive = false;

    const duration = Date.now() - sessionStartTime;
    sessionStartTime = 0;

    if (duration < CONFIG.MIN_VALID_SESSION_MS || duration > CONFIG.MAX_VALID_SESSION_MS) return;

    sessionHistory.push(duration);
    if (sessionHistory.length > CONFIG.SESSION_HISTORY_MAX) sessionHistory.shift();

    await invoke('save_session_history', { history: sessionHistory });
}

function getMinSessionMs() {
    return sessionHistory.length > 0 ? Math.min(...sessionHistory) : null;
}

// ── Re-login Timer ────────────────────────────────────────────────────────────
function scheduleRelogin() {
    if (reloginTimer) clearTimeout(reloginTimer);

    const minMs = getMinSessionMs();
    if (minMs === null) return null;

    const elapsed = Date.now() - sessionStartTime;
    const delay   = minMs - elapsed - CONFIG.PRE_EXPIRY_BUFFER_MS;
    const fireAt  = new Date(Date.now() + Math.max(delay, 0));

    if (delay <= 0) {
        // Already past predicted expiry → re-login immediately
        wasProactive = true;
        doLogin();
        return null;
    }

    reloginTimer = setTimeout(async () => {
        wasProactive = true;
        await doLogin();
    }, delay);

    return fireAt;
}

// ── UI State Transitions ──────────────────────────────────────────────────────
function showConnected(reloginAt) {
    document.getElementById('card-login').style.display   = 'none';
    document.getElementById('card-status').style.display  = 'block';

    // Show tray status hint for desktop
    const isMobile = navigator.userAgent.includes('Android');
    document.getElementById('tray-hint').textContent = isMobile
        ? 'Keep the app running for auto re-login.'
        : 'Running in tray — auto re-login armed.';

    const timeStr = reloginAt
        ? reloginAt.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
        : '—';
    document.getElementById('next-relogin').textContent = timeStr;

    const minMs = getMinSessionMs();
    document.getElementById('min-session').textContent  = minMs ? fmtDuration(minMs) : 'learning…';

    const histEl = document.getElementById('history-stat');
    histEl.innerHTML = sessionHistory.length < 3
        ? `<span style="color:#d97706">⚠ Collecting data (${sessionHistory.length}/${CONFIG.SESSION_HISTORY_MAX})</span>`
        : `<span style="color:#059669">📊 ${sessionHistory.length} sessions recorded</span>`;
}

function showLogin() {
    document.getElementById('card-status').style.display = 'none';
    document.getElementById('card-login').style.display  = 'block';
}

// ── Login Action ──────────────────────────────────────────────────────────────
async function doLogin() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;

    if (!username || !password) {
        setStatus('Please enter username and password.', 'error');
        return;
    }

    const btn = document.getElementById('btn-login');
    btn.disabled    = true;
    btn.textContent = 'Connecting…';
    setStatus('Connecting to captive portal…', 'info');

    // Retry cooldown check
    const timeSinceLast = Date.now() - lastAttemptTime;
    if (timeSinceLast < CONFIG.FAILED_RETRY_THRESHOLD_MS && lastAttemptTime !== 0) {
        setStatus('Previous login failed — please check your password.', 'error');
        btn.disabled    = false;
        btn.textContent = 'Connect';
        return;
    }

    lastAttemptTime = Date.now();

    try {
        // Save credentials locally
        await invoke('save_credentials', { username, password });

        // Record session end (skip if proactive renewal)
        await recordSessionEnd();

        // Native HTTP POST login via Rust backend
        const success = await invoke('login_to_portal', { username, password });

        if (success) {
            lastAttemptTime  = 0; // Success → reset cooldown
            sessionStartTime = Date.now();
            const reloginAt  = scheduleRelogin();
            showConnected(reloginAt);
        } else {
            setStatus('Login failed — please check your username and password.', 'error');
            btn.disabled    = false;
            btn.textContent = 'Connect';
        }
    } catch (e) {
        setStatus(`Error: ${e}`, 'error');
        btn.disabled    = false;
        btn.textContent = 'Connect';
    }
}

// ── Application Initialization ────────────────────────────────────────────────
async function init() {
    await loadHistory();

    // Load saved credentials
    const creds = await invoke('load_credentials').catch(() => ({ username: '', password: '' }));
    if (creds.username) document.getElementById('username').value = creds.username;
    if (creds.password) document.getElementById('password').value = creds.password;

    // Check network connectivity → arm timer if already connected
    try {
        const status = await invoke('check_connectivity');
        if (status === 'connected' && creds.username && creds.password) {
            sessionStartTime = Date.now();
            const reloginAt  = scheduleRelogin();
            showConnected(reloginAt);
            return;
        }
    } catch { /* Portal probe failed → show login screen */ }

    // Auto-login if credentials are saved
    if (creds.username && creds.password) {
        setStatus('Auto-logging in with saved credentials…', 'info');
        setTimeout(doLogin, 500);
    }
}

// ── Event Bindings ────────────────────────────────────────────────────────────
document.getElementById('btn-login').addEventListener('click', doLogin);
document.getElementById('password').addEventListener('keydown', e => {
    if (e.key === 'Enter') doLogin();
});
document.getElementById('btn-logout').addEventListener('click', () => {
    if (reloginTimer) clearTimeout(reloginTimer);
    sessionStartTime = 0;
    showLogin();
    clearStatus();
    document.getElementById('btn-login').disabled    = false;
    document.getElementById('btn-login').textContent = 'Connect';
});

init();
