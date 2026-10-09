// main.js — wifiLogin.js 로직을 Tauri invoke API로 이식
const invoke = window.__TAURI__?.core?.invoke ?? window.__TAURI__?.invoke;

// ── Config (wifiLogin.js와 동일) ──────────────────────────────────────────────
const CONFIG = {
    FAILED_RETRY_THRESHOLD_MS: 15_000,
    PRE_EXPIRY_BUFFER_MS: 5 * 60_000,
    SESSION_HISTORY_MAX: 30,
    MIN_VALID_SESSION_MS: 60_000,
    MAX_VALID_SESSION_MS: 48 * 3_600_000,
};

// ── 인메모리 상태 ─────────────────────────────────────────────────────────────
let sessionStartTime = 0;
let sessionHistory   = [];   // ms 단위 세션 지속시간 배열
let reloginTimer     = null;
let lastAttemptTime  = 0;
let wasProactive     = false;

// ── 헬퍼 ─────────────────────────────────────────────────────────────────────
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

// ── 세션 히스토리 ─────────────────────────────────────────────────────────────
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

// ── 재로그인 타이머 ───────────────────────────────────────────────────────────
function scheduleRelogin() {
    if (reloginTimer) clearTimeout(reloginTimer);

    const minMs = getMinSessionMs();
    if (minMs === null) return null;

    const elapsed = Date.now() - sessionStartTime;
    const delay   = minMs - elapsed - CONFIG.PRE_EXPIRY_BUFFER_MS;
    const fireAt  = new Date(Date.now() + Math.max(delay, 0));

    if (delay <= 0) {
        // 이미 만료 예상 → 즉시 재로그인
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

// ── UI 상태 전환 ──────────────────────────────────────────────────────────────
function showConnected(reloginAt) {
    document.getElementById('card-login').style.display   = 'none';
    document.getElementById('card-status').style.display  = 'block';

    // 데스크탑에서는 트레이 힌트 표시
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

// ── 로그인 실행 ───────────────────────────────────────────────────────────────
async function doLogin() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;

    if (!username || !password) {
        setStatus('아이디와 비밀번호를 입력해주세요.', 'error');
        return;
    }

    const btn = document.getElementById('btn-login');
    btn.disabled    = true;
    btn.textContent = 'Connecting…';
    setStatus('포털에 연결 중…', 'info');

    // 재시도 쿨다운 체크
    const timeSinceLast = Date.now() - lastAttemptTime;
    if (timeSinceLast < CONFIG.FAILED_RETRY_THRESHOLD_MS && lastAttemptTime !== 0) {
        setStatus('이전 로그인 실패 — 비밀번호를 확인해주세요.', 'error');
        btn.disabled    = false;
        btn.textContent = 'Connect';
        return;
    }

    lastAttemptTime = Date.now();

    try {
        // 자격증명 저장
        await invoke('save_credentials', { username, password });

        // 세션 종료 기록 (재로그인이면 스킵)
        await recordSessionEnd();

        // Rust → 직접 HTTP POST 로그인
        const success = await invoke('login_to_portal', { username, password });

        if (success) {
            lastAttemptTime  = 0; // 성공 → 쿨다운 리셋
            sessionStartTime = Date.now();
            const reloginAt  = scheduleRelogin();
            showConnected(reloginAt);
        } else {
            setStatus('로그인 실패 — 아이디/비밀번호를 확인하세요.', 'error');
            btn.disabled    = false;
            btn.textContent = 'Connect';
        }
    } catch (e) {
        setStatus(`오류: ${e}`, 'error');
        btn.disabled    = false;
        btn.textContent = 'Connect';
    }
}

// ── 앱 시작 ───────────────────────────────────────────────────────────────────
async function init() {
    await loadHistory();

    // 저장된 자격증명 불러오기
    const creds = await invoke('load_credentials').catch(() => ({ username: '', password: '' }));
    if (creds.username) document.getElementById('username').value = creds.username;
    if (creds.password) document.getElementById('password').value = creds.password;

    // 네트워크 상태 확인 → 이미 연결됐으면 바로 타이머 설정
    try {
        const status = await invoke('check_connectivity');
        if (status === 'connected' && creds.username && creds.password) {
            sessionStartTime = Date.now(); // 정확한 시작 시간 미지수, 현재부터 계산
            const reloginAt  = scheduleRelogin();
            showConnected(reloginAt);
            return;
        }
    } catch { /* 포털 감지 실패 → 로그인 화면 */ }

    // 저장된 자격증명 있으면 자동 로그인 시도
    if (creds.username && creds.password) {
        setStatus('저장된 계정으로 자동 로그인 중…', 'info');
        setTimeout(doLogin, 500);
    }
}

// ── 이벤트 바인딩 ─────────────────────────────────────────────────────────────
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
