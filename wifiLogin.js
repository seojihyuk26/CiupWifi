// ==UserScript==
// @name            cite university wifi auto login script
// @name:fr         Cité université wifi auto login script
// @namespace       seojihyuk@university
// @match           http://10.254.0.254:*/*
// @match           http://captive.apple.com/*
// @match           http://detectportal.firefox.com/canonical.html
// @match           http://www.msftconnecttest.com/redirect
// @match           http://www.gstatic.com/generate_204
// @match           http://edge-http.microsoft.com/captiveportal/generate_204
// @version         3.0.0
// @license         MIT
// @author          seojihyuk
// @grant           GM_getValue
// @grant           GM_setValue
// @description     Auto-login + statistical session-expiry prediction for Cité Universitaire WiFi.
//                  Tracks natural session durations, predicts minimum lifetime, and proactively
//                  re-opens the login portal before the session expires.
// ==/UserScript==
'use strict';

// ── Config ────────────────────────────────────────────────────────────────────
const CONFIG = {
    /** Reload within this window after a login attempt → treat as failed login. */
    FAILED_RETRY_THRESHOLD_MS: 15_000,
    /** Open login portal this many ms BEFORE predicted session expiry. */
    PRE_EXPIRY_BUFFER_MS: 5 * 60_000,
    /** Maximum number of session duration records to keep. */
    SESSION_HISTORY_MAX: 30,
    /** Ignore sessions shorter than this (noise / failed logins that slipped through). */
    MIN_VALID_SESSION_MS: 60_000,
    /** Ignore sessions longer than this (anomaly / machine was asleep). */
    MAX_VALID_SESSION_MS: 48 * 3_600_000,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────
const KEY = {
    USERNAME: "saved_username",
    PASSWORD: "saved_password",
    ATTEMPT: "lastLoginAttemptTime",
    SES_START: "sessionStartTime",
    SES_HIST: "sessionDurations",
    PROACTIVE: "proactiveRelogin",   // flag: true when WE initiated re-login (not expiry)
};

// ── Helpers ───────────────────────────────────────────────────────────────────

function fmtDuration(ms) {
    const m = Math.round(ms / 60_000);
    return m >= 60 ? `${(m / 60).toFixed(1)} h` : `${m} min`;
}

// ── Session History ───────────────────────────────────────────────────────────

function loadHistory() {
    try { return JSON.parse(GM_getValue(KEY.SES_HIST, "[]")); }
    catch { return []; }
}

/**
 * Records how long the last session lasted.
 * Skipped when the re-login was proactively triggered by us (not a real expiry),
 * so statistics only reflect genuine WiFi session lengths.
 */
function recordSessionEnd() {
    const wasProactive = GM_getValue(KEY.PROACTIVE, false);
    GM_setValue(KEY.PROACTIVE, false); // always clear the flag

    const start = GM_getValue(KEY.SES_START, 0);
    GM_setValue(KEY.SES_START, 0);

    if (wasProactive || !start) return; // proactive re-login → not a real expiry, skip

    const duration = Date.now() - start;
    if (duration < CONFIG.MIN_VALID_SESSION_MS || duration > CONFIG.MAX_VALID_SESSION_MS) {
        console.log(`[AutoLogin] Session duration ${fmtDuration(duration)} out of valid range – not recorded.`);
        return;
    }

    const history = loadHistory();
    history.push(duration);
    if (history.length > CONFIG.SESSION_HISTORY_MAX) history.shift();
    GM_setValue(KEY.SES_HIST, JSON.stringify(history));
    console.log(`[AutoLogin] Session ended naturally (${fmtDuration(duration)}). ${history.length} records stored.`);
}

/**
 * Returns the statistical minimum session duration from history,
 * which is the safest predictor for when to trigger a proactive re-login.
 * Returns null when there is not enough data yet.
 */
function getMinSessionMs() {
    const h = loadHistory();
    return h.length > 0 ? Math.min(...h) : null;
}

// ── Proactive Re-login Timer ──────────────────────────────────────────────────

/**
 * Schedules a navigation to the captive portal before the session expires.
 * Uses the minimum observed session duration as the expiry estimate.
 * The tab must remain open for this timer to fire.
 */
function scheduleRelogin() {
    const minMs = getMinSessionMs();
    if (minMs === null) {
        console.log("[AutoLogin] No session history yet – proactive timer not set.");
        return null;
    }

    const sessionStart = GM_getValue(KEY.SES_START, Date.now());
    const elapsed = Date.now() - sessionStart;
    const delay = minMs - elapsed - CONFIG.PRE_EXPIRY_BUFFER_MS;
    const reloginAt = new Date(Date.now() + Math.max(delay, 0));

    console.log(
        `[AutoLogin] History min session: ${fmtDuration(minMs)}.` +
        ` Proactive re-login scheduled at ${reloginAt.toLocaleTimeString()}.`
    );

    if (delay <= 0) {
        // Already past predicted expiry – re-login immediately
        GM_setValue(KEY.PROACTIVE, true);
        window.location.href = "http://10.254.0.254/";
        return null;
    }

    const timerId = setTimeout(() => {
        GM_setValue(KEY.PROACTIVE, true);
        window.location.href = "http://10.254.0.254/";
    }, delay);

    return { reloginAt, minMs, timerId };
}

// ── Status Overlay ────────────────────────────────────────────────────────────

/**
 * Replaces the connectivity-check page content with a minimal status card.
 * The tab is kept open so the re-login setTimeout remains active.
 */
function showStatusOverlay({ reloginAt, minMs }) {
    const timeStr = reloginAt
        ? reloginAt.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
        : "—";
    const histLen = loadHistory().length;
    const note = histLen < 3
        ? `<span style="color:#d97706">⚠ Collecting data (${histLen}/${CONFIG.SESSION_HISTORY_MAX} sessions)</span>`
        : `<span style="color:#059669">📊 ${histLen} sessions recorded</span>`;

    document.documentElement.innerHTML = `
        <html><head><title>WiFi – Connected</title>
        <meta name="viewport" content="width=device-width,initial-scale=1">
        <style>
            body {
                margin: 0; display: flex; align-items: center; justify-content: center;
                min-height: 100vh; background: #f8fafc;
                font-family: system-ui, -apple-system, sans-serif; color: #1e293b;
            }
            .card {
                background: white; border-radius: 16px;
                box-shadow: 0 4px 24px rgba(0,0,0,.08);
                padding: 40px 48px; text-align: center; max-width: 360px;
            }
            .icon { font-size: 52px; margin-bottom: 12px; }
            h1 { margin: 0 0 6px; font-size: 22px; font-weight: 700; }
            .sub { color: #64748b; font-size: 14px; margin-bottom: 20px; }
            .stat { background: #f1f5f9; border-radius: 10px; padding: 12px 16px;
                    font-size: 13px; color: #475569; margin-top: 8px; text-align: left; }
            .stat strong { color: #1e293b; }
        </style></head><body>
        <div class="card">
            <div class="icon">✅</div>
            <h1>WiFi Connected</h1>
            <p class="sub">Keep this tab open – auto re-login is armed.</p>
            <div class="stat">🔁 Next re-login: <strong>${timeStr}</strong></div>
            <div class="stat">⏱ Min session length: <strong>${minMs ? fmtDuration(minMs) : "learning…"}</strong></div>
            <div class="stat">${note}</div>
        </div>
        </body></html>`;
}

// ── Login Page Handler (hostname = 10.254.0.254) ──────────────────────────────

function handleLoginPage() {
    // Arriving here means the previous session has ended → record its duration.
    recordSessionEnd();

    const usernameDom = document.querySelector("#ft_un");
    const passwordDom = document.querySelector("#ft_pd");
    const button = document.querySelector(".fer > input")
        || document.querySelector("input[type='submit']");

    if (!usernameDom || !passwordDom || !button) return;

    // Persist credentials whenever the user manually clicks submit.
    button.addEventListener("click", () => {
        if (usernameDom.value && passwordDom.value) {
            GM_setValue(KEY.USERNAME, usernameDom.value);
            GM_setValue(KEY.PASSWORD, passwordDom.value);
            GM_setValue(KEY.ATTEMPT, 0); // reset cooldown on manual submission
        }
    });

    const savedUsername = GM_getValue(KEY.USERNAME, "");
    const savedPassword = GM_getValue(KEY.PASSWORD, "");
    const timeSinceLast = Date.now() - GM_getValue(KEY.ATTEMPT, 0);
    const isFailedRetry = timeSinceLast < CONFIG.FAILED_RETRY_THRESHOLD_MS;

    if (savedUsername && savedPassword && !isFailedRetry) {
        usernameDom.value = savedUsername;
        passwordDom.value = savedPassword;
        GM_setValue(KEY.ATTEMPT, Date.now());
        setTimeout(() => button.click(), 300);

    } else if (isFailedRetry) {
        console.warn("[AutoLogin] Previous login failed – halting auto-login for manual correction.");
        passwordDom.value = "";
        passwordDom.focus();
    }
}

// ── Connectivity-Check Page Handler (all other @match URLs) ──────────────────

function handleConnectivityPage() {
    const xhr = new XMLHttpRequest();
    xhr.open("GET", window.location.href, true);
    xhr.setRequestHeader("Cache-Control", "no-cache");

    xhr.onreadystatechange = function () {
        if (xhr.readyState !== 4) return;

        if (xhr.status === 200) {
            const doc = new DOMParser().parseFromString(xhr.response, "text/html");
            // FIX: html.title is undefined on a parsed document; use querySelector instead.
            const title = doc.querySelector("title")?.textContent?.trim() ?? "";
            const body = doc.body?.innerText?.trim() ?? "";
            const isSuccess = title === "Success" || body.startsWith("Success");

            if (isSuccess) {
                GM_setValue(KEY.ATTEMPT, 0);             // clear failure cooldown
                GM_setValue(KEY.SES_START, Date.now()); // record session start timestamp

                const timerInfo = scheduleRelogin();
                showStatusOverlay(timerInfo ?? { reloginAt: null, minMs: getMinSessionMs() });
                // ⚠ Tab intentionally NOT closed – closing would destroy the setTimeout timer.

            } else {
                redirectToCaptivePortal();
            }

        } else {
            redirectToCaptivePortal();
        }
    };

    // FIX: handle CORS / network errors that set status=0 and skip onreadystatechange.
    xhr.onerror = function () {
        console.warn("[AutoLogin] XHR failed (CORS or network error) – redirecting to portal.");
        redirectToCaptivePortal();
    };

    xhr.send();
}

// ── Redirect Utility ──────────────────────────────────────────────────────────

function redirectToCaptivePortal() {
    const CAPTIVE_HOSTNAMES = [
        "10.254.0.254", "captive.apple.com", "detectportal.firefox.com",
        "msftconnecttest.com", "gstatic.com", "edge-http.microsoft.com",
    ];
    const isCaptivePage = CAPTIVE_HOSTNAMES.some(h => window.location.href.includes(h));
    if (!isCaptivePage) {
        setTimeout(() => {
            window.location.href = "http://captive.apple.com/hotspot-detect.html";
        }, 3000);
    }
}

// ── Entry Point ───────────────────────────────────────────────────────────────

if (window.location.hostname === "10.254.0.254") {
    handleLoginPage();
} else {
    handleConnectivityPage();
}
