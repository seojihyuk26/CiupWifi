package org.ciup.wifi

import android.app.PendingIntent
import android.app.job.JobInfo
import android.app.job.JobScheduler
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.net.ConnectivityManager
import android.net.LinkProperties
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.os.Build
import android.util.Log
import org.json.JSONObject
import java.net.Inet4Address
import java.util.Locale
import java.util.concurrent.atomic.AtomicBoolean

object WifiTriggerManager {
    private const val TAG = "CiupWifi"
    const val PREFS_NAME = "ciupwifi_prefs"
    const val KEY_USERNAME = "username"
    const val KEY_PASSWORD = "password"
    const val KEY_AUTO_ENABLED = "auto_enabled"
    const val KEY_USER_PAUSED = "user_paused"
    const val KEY_PAUSED_NET_HANDLE = "paused_net_handle"
    const val KEY_AUTH_FAILED = "auth_failed"
    const val KEY_LAST_PORTAL = "last_portal_url"
    const val KEY_LAST_LOGOUT = "last_logout_url"
    const val KEY_LAST_ERROR = "last_error"
    const val KEY_LAST_AUTH_NET_HANDLE = "last_auth_net_handle"
    const val KEY_KNOWN_GATEWAYS = "known_ciup_gateways"

    const val ACTION_CAPTIVE_EVENT = "org.ciup.wifi.ACTION_CAPTIVE_EVENT"
    const val ACTION_WIFI_EVENT = "org.ciup.wifi.ACTION_WIFI_EVENT"
    private const val REQ_CAPTIVE_PI = 2610
    private const val REQ_WIFI_PI = 2612
    private const val JOB_ID_RETRY = 2613

    private val authInFlight = AtomicBoolean(false)

    private fun prefs(context: Context): SharedPreferences =
        context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    private fun buildCaptiveRequest(): NetworkRequest =
        NetworkRequest.Builder()
            .addTransportType(NetworkCapabilities.TRANSPORT_WIFI)
            .addCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL)
            .build()

    private fun buildWifiRequest(): NetworkRequest =
        NetworkRequest.Builder()
            .addTransportType(NetworkCapabilities.TRANSPORT_WIFI)
            .build()

    private fun getMutablePendingIntent(context: Context, action: String, reqCode: Int): PendingIntent {
        val intent = Intent(context, WifiEventReceiver::class.java).apply {
            this.action = action
        }
        // Android 12+ requires FLAG_MUTABLE so ConnectivityManager can attach EXTRA_NETWORK
        val flags = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_MUTABLE
        } else {
            PendingIntent.FLAG_UPDATE_CURRENT
        }
        return PendingIntent.getBroadcast(context, reqCode, intent, flags)
    }

    fun register(context: Context) {
        val appCtx = context.applicationContext
        val p = prefs(appCtx)
        if (!p.getBoolean(KEY_AUTO_ENABLED, true)) return

        val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as? ConnectivityManager ?: return
        try {
            cm.registerNetworkCallback(
                buildCaptiveRequest(),
                getMutablePendingIntent(appCtx, ACTION_CAPTIVE_EVENT, REQ_CAPTIVE_PI)
            )
        } catch (e: Exception) {
            Log.w(TAG, "Failed to register captive network callback: ${e.message}")
        }
        try {
            cm.registerNetworkCallback(
                buildWifiRequest(),
                getMutablePendingIntent(appCtx, ACTION_WIFI_EVENT, REQ_WIFI_PI)
            )
        } catch (e: Exception) {
            Log.w(TAG, "Failed to register wifi network callback: ${e.message}")
        }
    }

    fun scheduleRetryJob(context: Context) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.LOLLIPOP) return
        val appCtx = context.applicationContext
        val p = prefs(appCtx)
        if (!p.getBoolean(KEY_AUTO_ENABLED, true)) return

        val js = appCtx.getSystemService(Context.JOB_SCHEDULER_SERVICE) as? JobScheduler ?: return
        val builder = JobInfo.Builder(
            JOB_ID_RETRY,
            ComponentName(appCtx, WifiAuthJobService::class.java)
        )
            .setRequiredNetworkType(JobInfo.NETWORK_TYPE_UNMETERED)
            .setMinimumLatency(6_000L)
            .setOverrideDeadline(30_000L)
        try {
            js.schedule(builder.build())
        } catch (e: Exception) {
            Log.w(TAG, "Failed to schedule retry job: ${e.message}")
        }
    }

    fun unregister(context: Context) {
        val appCtx = context.applicationContext
        val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as? ConnectivityManager
        for ((action, code) in listOf(
            ACTION_CAPTIVE_EVENT to REQ_CAPTIVE_PI,
            ACTION_WIFI_EVENT to REQ_WIFI_PI
        )) {
            try {
                cm?.unregisterNetworkCallback(getMutablePendingIntent(appCtx, action, code))
            } catch (_: Exception) {
            }
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            val js = appCtx.getSystemService(Context.JOB_SCHEDULER_SERVICE) as? JobScheduler
            try {
                js?.cancel(JOB_ID_RETRY)
            } catch (_: Exception) {
            }
        }
    }

    fun extractGateways(cm: ConnectivityManager, network: Network?): List<String> {
        if (network == null) return emptyList()
        val lp = try {
            cm.getLinkProperties(network)
        } catch (_: Exception) {
            null
        }
        return lp?.routes
            ?.mapNotNull { it.gateway as? Inet4Address }
            ?.mapNotNull { it.hostAddress }
            ?.distinct()
            ?: emptyList()
    }

    fun isCiupGateway(gateways: List<String>, p: SharedPreferences): Boolean {
        if (gateways.any { it.startsWith("10.254.") }) return true
        val known = p.getStringSet(KEY_KNOWN_GATEWAYS, emptySet()) ?: emptySet()
        return gateways.any { known.contains(it) }
    }

    private fun isCiupPortal(
        gateways: List<String>,
        portalUrl: String?,
        html: String?,
        p: SharedPreferences
    ): Boolean {
        if (isCiupGateway(gateways, p)) return true
        val urlLower = (portalUrl ?: "").lowercase(Locale.ROOT)
        if (urlLower.contains("10.254.") ||
            urlLower.contains("fgtauth") ||
            urlLower.contains("wificity") ||
            urlLower.contains("ciup")
        ) {
            return true
        }
        val htmlLower = (html ?: "").lowercase(Locale.ROOT)
        return htmlLower.contains("ft_un") && htmlLower.contains("ft_pd")
    }

    fun handleWifiEvent(context: Context, network: Network, isCaptiveEvent: Boolean) {
        val appCtx = context.applicationContext
        val p = prefs(appCtx)
        if (!p.getBoolean(KEY_AUTO_ENABLED, true)) return

        val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val caps = cm.getNetworkCapabilities(network)
        var gateways = extractGateways(cm, network)

        if (!isCaptiveEvent) {
            val hasCaptive = caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) == true
            val isValidated = caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) == true
            // Fast 1ms exit when sitting on normal home Wi-Fi that is already validated
            if (isValidated && !hasCaptive && !isCiupGateway(gateways, p)) {
                return
            }
            if (gateways.isEmpty()) {
                try {
                    Thread.sleep(700)
                } catch (_: InterruptedException) {
                }
                gateways = extractGateways(cm, network)
            }
        }

        handleCaptiveNetwork(appCtx, network, forceUserInitiated = false)
    }

    fun handleCaptiveNetwork(
        context: Context,
        network: Network,
        forceUserInitiated: Boolean = false
    ): String {
        val appCtx = context.applicationContext
        val p = prefs(appCtx)
        val user = p.getString(KEY_USERNAME, "") ?: ""
        val pass = p.getString(KEY_PASSWORD, "") ?: ""

        if (user.isEmpty() || pass.isEmpty()) {
            return JSONObject().apply {
                put("ok", false)
                put("auth_failed", false)
                put("message", "Please enter username and password.")
            }.toString()
        }

        if (!forceUserInitiated && !p.getBoolean(KEY_AUTO_ENABLED, true)) {
            return JSONObject().apply {
                put("ok", false)
                put("auth_failed", false)
                put("message", "Auto-login disabled.")
            }.toString()
        }

        val netHandle = network.networkHandle
        val paused = p.getBoolean(KEY_USER_PAUSED, false)
        val pausedHandle = p.getLong(KEY_PAUSED_NET_HANDLE, -1L)
        if (paused && pausedHandle == netHandle && !forceUserInitiated) {
            return JSONObject().apply {
                put("ok", false)
                put("auth_failed", false)
                put("message", "Paused by user.")
            }.toString()
        }

        // Rejoined WifiCity (different network handle) or user pressed Connect -> clear pause/auth errors
        if (forceUserInitiated || (paused && pausedHandle != netHandle)) {
            p.edit()
                .putBoolean(KEY_USER_PAUSED, false)
                .putLong(KEY_PAUSED_NET_HANDLE, -1L)
                .putBoolean(KEY_AUTH_FAILED, false)
                .remove(KEY_LAST_ERROR)
                .apply()
        } else if (p.getBoolean(KEY_AUTH_FAILED, false)) {
            return JSONObject().apply {
                put("ok", false)
                put("auth_failed", true)
                put("message", p.getString(KEY_LAST_ERROR, "Invalid credentials.") ?: "Invalid credentials.")
            }.toString()
        }

        if (!forceUserInitiated && !authInFlight.compareAndSet(false, true)) {
            return JSONObject().apply {
                put("ok", false)
                put("auth_failed", false)
                put("message", "Authentication already in progress.")
            }.toString()
        }

        try {
            val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
            val gateways = extractGateways(cm, network)
            val probe = WifiPortalClient.probeConnectivity(network, gateways)

            when (probe) {
                is WifiPortalClient.ProbeResult.Online -> {
                    if (isCiupGateway(gateways, p) || p.getLong(KEY_LAST_AUTH_NET_HANDLE, -1L) == netHandle || forceUserInitiated) {
                        cm.reportNetworkConnectivity(network, true)
                        p.edit()
                            .putLong(KEY_LAST_AUTH_NET_HANDLE, netHandle)
                            .putBoolean(KEY_AUTH_FAILED, false)
                            .remove(KEY_LAST_ERROR)
                            .apply()
                        return JSONObject().apply {
                            put("ok", true)
                            put("auth_failed", false)
                            put("message", "Connected to WifiCity.")
                        }.toString()
                    }
                    return JSONObject().apply {
                        put("ok", false)
                        put("auth_failed", false)
                        put("message", "Not connected to WifiCity.")
                    }.toString()
                }
                is WifiPortalClient.ProbeResult.Captive -> {
                    if (!forceUserInitiated && !isCiupPortal(gateways, probe.portalUrl, probe.initialHtml, p)) {
                        return JSONObject().apply {
                            put("ok", false)
                            put("auth_failed", false)
                            put("message", "Other Wi-Fi captive portal ignored.")
                        }.toString()
                    }
                }
                is WifiPortalClient.ProbeResult.Offline -> {
                    if (!forceUserInitiated && !isCiupGateway(gateways, p)) {
                        return JSONObject().apply {
                            put("ok", false)
                            put("auth_failed", false)
                            put("message", "Captive portal not detected on WifiCity.")
                        }.toString()
                    }
                    if (!forceUserInitiated) {
                        scheduleRetryJob(appCtx)
                        return JSONObject().apply {
                            put("ok", false)
                            put("auth_failed", false)
                            put("message", "Waiting for portal on WifiCity.")
                        }.toString()
                    }
                }
            }

            val res = WifiPortalClient.loginWithProbe(network, user, pass, gateways, probe)
            if (res.ok) {
                cm.reportNetworkConnectivity(network, true)
                val updatedKnown = (p.getStringSet(KEY_KNOWN_GATEWAYS, emptySet()) ?: emptySet()).toMutableSet()
                updatedKnown.addAll(gateways)
                p.edit().apply {
                    putBoolean(KEY_USER_PAUSED, false)
                    putLong(KEY_PAUSED_NET_HANDLE, -1L)
                    putLong(KEY_LAST_AUTH_NET_HANDLE, netHandle)
                    putBoolean(KEY_AUTH_FAILED, false)
                    remove(KEY_LAST_ERROR)
                    putStringSet(KEY_KNOWN_GATEWAYS, updatedKnown)
                    if (res.portalUrl != null) putString(KEY_LAST_PORTAL, res.portalUrl)
                    if (res.logoutUrl != null) putString(KEY_LAST_LOGOUT, res.logoutUrl)
                }.apply()
            } else if (res.authFailed) {
                p.edit()
                    .putBoolean(KEY_AUTH_FAILED, true)
                    .putString(KEY_LAST_ERROR, res.message)
                    .apply()
            } else {
                p.edit().putString(KEY_LAST_ERROR, res.message).apply()
                if (!forceUserInitiated) {
                    scheduleRetryJob(appCtx)
                }
            }

            return JSONObject().apply {
                put("ok", res.ok)
                put("auth_failed", res.authFailed)
                put("portal_url", res.portalUrl ?: JSONObject.NULL)
                put("message", res.message)
            }.toString()
        } finally {
            if (!forceUserInitiated) {
                authInFlight.set(false)
            }
        }
    }

    fun disconnectWifiCity(context: Context, wifiNetwork: Network?) {
        val appCtx = context.applicationContext
        val p = prefs(appCtx)
        val logoutUrl = p.getString(KEY_LAST_LOGOUT, null)
        val handle = wifiNetwork?.networkHandle ?: -1L
        p.edit()
            .putBoolean(KEY_USER_PAUSED, true)
            .putLong(KEY_PAUSED_NET_HANDLE, handle)
            .apply()

        if (wifiNetwork != null) {
            val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
            if (!logoutUrl.isNullOrBlank()) {
                WifiPortalClient.logout(wifiNetwork, logoutUrl)
            }
            cm.reportNetworkConnectivity(wifiNetwork, false)
        }
    }

    fun buildStatusJson(
        context: Context,
        wifiNet: Network?,
        wifiCaps: NetworkCapabilities?,
        wifiLp: LinkProperties?,
        isCellularDefault: Boolean
    ): String {
        val p = prefs(context.applicationContext)
        val paused = p.getBoolean(KEY_USER_PAUSED, false)
        val pausedHandle = p.getLong(KEY_PAUSED_NET_HANDLE, -1L)
        val auto = p.getBoolean(KEY_AUTO_ENABLED, true)
        val authFailed = p.getBoolean(KEY_AUTH_FAILED, false)
        val lastPortal = p.getString(KEY_LAST_PORTAL, null)
        val lastError = p.getString(KEY_LAST_ERROR, null)
        val lastAuthHandle = p.getLong(KEY_LAST_AUTH_NET_HANDLE, -1L)

        if (wifiNet == null) {
            val state = if (isCellularDefault) "not_on_wificity" else "disconnected"
            val detail = if (isCellularDefault) "On Mobile Data (Not on WifiCity)." else "Wi-Fi is disconnected."
            return JSONObject().apply {
                put("state", state)
                put("ssid", JSONObject.NULL)
                put("is_wifi", false)
                put("is_cellular", isCellularDefault)
                put("is_wificity", false)
                put("user_paused", false)
                put("auto_enabled", auto)
                put("auth_failed", authFailed)
                put("portal_url", lastPortal ?: JSONObject.NULL)
                put("detail", detail)
            }.toString()
        }

        val gateways = wifiLp?.routes
            ?.mapNotNull { it.gateway as? Inet4Address }
            ?.mapNotNull { it.hostAddress }
            ?.distinct()
            ?: emptyList()

        val netHandle = wifiNet.networkHandle
        val isCiupGw = isCiupGateway(gateways, p)
        val isAuthNet = lastAuthHandle == netHandle
        val hasCaptive = wifiCaps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) == true
        val isValidated = wifiCaps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) == true
        val isPausedThisNet = paused && pausedHandle == netHandle
        val isWifiCity = isCiupGw || isAuthNet || hasCaptive

        val state = when {
            isPausedThisNet -> "paused"
            (isCiupGw || isAuthNet) && isValidated && !hasCaptive -> "wifi_city_online"
            isWifiCity -> "wifi_city_captive"
            else -> "not_on_wificity"
        }

        val detail = when (state) {
            "paused" -> "Disconnected by user. Tap Connect or rejoin WifiCity to resume."
            "not_on_wificity" -> "Connected to another Wi-Fi network (Not WifiCity)."
            "wifi_city_captive" -> lastError
            else -> null
        }

        return JSONObject().apply {
            put("state", state)
            put("ssid", if (isCiupGw || isAuthNet) "WifiCity" else JSONObject.NULL)
            put("is_wifi", true)
            put("is_cellular", false)
            put("is_wificity", isWifiCity)
            put("user_paused", isPausedThisNet)
            put("auto_enabled", auto)
            put("auth_failed", authFailed)
            put("portal_url", lastPortal ?: JSONObject.NULL)
            put("detail", detail ?: JSONObject.NULL)
        }.toString()
    }
}
