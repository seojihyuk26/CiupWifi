package org.ciup.wifi

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.net.LinkProperties
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.os.Build
import android.provider.Settings
import android.webkit.JavascriptInterface
import android.webkit.WebView
import org.json.JSONObject
import java.util.concurrent.Executors

class WifiBridge(
    private val activity: Activity,
    private val webView: WebView
) {
    private val cm = activity.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
    private val executor = Executors.newSingleThreadExecutor()

    @Volatile
    private var currentWifiNetwork: Network? = null

    @Volatile
    private var currentWifiCaps: NetworkCapabilities? = null

    @Volatile
    private var currentWifiLp: LinkProperties? = null

    @Volatile
    private var isCellularDefault: Boolean = false

    @Volatile
    private var registered = false

    private val wifiCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            currentWifiNetwork = network
            currentWifiCaps = cm.getNetworkCapabilities(network)
            currentWifiLp = cm.getLinkProperties(network)
            pushStatus()
        }

        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return
            currentWifiNetwork = network
            currentWifiCaps = caps
            pushStatus()
        }

        override fun onLinkPropertiesChanged(network: Network, linkProperties: LinkProperties) {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return
            currentWifiNetwork = network
            currentWifiLp = linkProperties
            pushStatus()
        }

        override fun onLost(network: Network) {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return
            currentWifiNetwork = null
            currentWifiCaps = null
            currentWifiLp = null
            pushStatus()
        }
    }

    private val defaultCallback = object : ConnectivityManager.NetworkCallback() {
        override fun onAvailable(network: Network) {
            val caps = cm.getNetworkCapabilities(network)
            isCellularDefault = caps?.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) == true &&
                caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) != true
            pushStatus()
        }

        override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) {
            isCellularDefault = caps.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) &&
                !caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)
            pushStatus()
        }

        override fun onLost(network: Network) {
            isCellularDefault = false
            pushStatus()
        }
    }

    fun startForegroundCallbacks() {
        if (registered) return
        registered = true
        refreshSnapshotFromCm()
        val req = NetworkRequest.Builder()
            .addTransportType(NetworkCapabilities.TRANSPORT_WIFI)
            .build()
        try {
            cm.registerNetworkCallback(req, wifiCallback)
            cm.registerDefaultNetworkCallback(defaultCallback)
        } catch (_: Exception) {
        }
        pushStatus()
    }

    fun stopForegroundCallbacks() {
        if (!registered) return
        registered = false
        try {
            cm.unregisterNetworkCallback(wifiCallback)
        } catch (_: Exception) {
        }
        try {
            cm.unregisterNetworkCallback(defaultCallback)
        } catch (_: Exception) {
        }
    }

    fun dispose() {
        stopForegroundCallbacks()
        executor.shutdownNow()
    }

    private fun refreshSnapshotFromCm() {
        val active = cm.activeNetwork
        val activeCaps = if (active != null) cm.getNetworkCapabilities(active) else null
        isCellularDefault = activeCaps?.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) == true &&
            activeCaps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) != true

        if (active != null && activeCaps?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true) {
            currentWifiNetwork = active
            currentWifiCaps = activeCaps
            currentWifiLp = cm.getLinkProperties(active)
        } else {
            @Suppress("DEPRECATION")
            val wifiNet = try {
                cm.allNetworks.firstOrNull { net ->
                    cm.getNetworkCapabilities(net)?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true
                }
            } catch (_: Exception) {
                null
            }
            currentWifiNetwork = wifiNet
            currentWifiCaps = if (wifiNet != null) cm.getNetworkCapabilities(wifiNet) else null
            currentWifiLp = if (wifiNet != null) cm.getLinkProperties(wifiNet) else null
        }
    }

    private fun pushStatus() {
        val json = WifiTriggerManager.buildStatusJson(
            activity,
            currentWifiNetwork,
            currentWifiCaps,
            currentWifiLp,
            isCellularDefault
        )
        val escaped = JSONObject.quote(json)
        webView.post {
            webView.evaluateJavascript(
                "window.__onAndroidWifiStatus && window.__onAndroidWifiStatus(JSON.parse($escaped));",
                null
            )
        }
    }

    @JavascriptInterface
    fun getNetworkStatus(): String {
        if (!registered) {
            refreshSnapshotFromCm()
        }
        return WifiTriggerManager.buildStatusJson(
            activity,
            currentWifiNetwork,
            currentWifiCaps,
            currentWifiLp,
            isCellularDefault
        )
    }

    @JavascriptInterface
    fun saveCredentials(username: String, password: String): String {
        val prefs = activity.getSharedPreferences(WifiTriggerManager.PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit()
            .putString(WifiTriggerManager.KEY_USERNAME, username)
            .putString(WifiTriggerManager.KEY_PASSWORD, password)
            .putBoolean(WifiTriggerManager.KEY_AUTH_FAILED, false)
            .remove(WifiTriggerManager.KEY_LAST_ERROR)
            .apply()
        WifiTriggerManager.register(activity)
        return JSONObject().apply { put("ok", true) }.toString()
    }

    @JavascriptInterface
    fun loadCredentials(): String {
        val prefs = activity.getSharedPreferences(WifiTriggerManager.PREFS_NAME, Context.MODE_PRIVATE)
        return JSONObject().apply {
            put("username", prefs.getString(WifiTriggerManager.KEY_USERNAME, "") ?: "")
            put("password", prefs.getString(WifiTriggerManager.KEY_PASSWORD, "") ?: "")
        }.toString()
    }

    @JavascriptInterface
    fun connectNow(requestId: String) {
        executor.execute {
            refreshSnapshotFromCm()
            val net = currentWifiNetwork
            val resJson = if (net == null) {
                JSONObject().apply {
                    put("ok", false)
                    put("auth_failed", false)
                    put("message", "Not connected to Wi-Fi. Switch to WifiCity first.")
                }.toString()
            } else {
                WifiTriggerManager.handleCaptiveNetwork(activity, net, forceUserInitiated = true)
            }

            refreshSnapshotFromCm()
            pushStatus()

            val reqQuoted = JSONObject.quote(requestId)
            val resQuoted = JSONObject.quote(resJson)
            webView.post {
                webView.evaluateJavascript(
                    "window.__onAndroidAsyncResult && window.__onAndroidAsyncResult($reqQuoted, JSON.parse($resQuoted));",
                    null
                )
            }
        }
    }

    @JavascriptInterface
    fun disconnectNow(requestId: String) {
        executor.execute {
            refreshSnapshotFromCm()
            WifiTriggerManager.disconnectWifiCity(activity, currentWifiNetwork)
            refreshSnapshotFromCm()
            pushStatus()

            val reqQuoted = JSONObject.quote(requestId)
            val resQuoted = JSONObject.quote(JSONObject().apply { put("ok", true) }.toString())
            webView.post {
                webView.evaluateJavascript(
                    "window.__onAndroidAsyncResult && window.__onAndroidAsyncResult($reqQuoted, JSON.parse($resQuoted));",
                    null
                )
            }
        }
    }

    @JavascriptInterface
    fun setAutoLogin(enabled: Boolean) {
        val prefs = activity.getSharedPreferences(WifiTriggerManager.PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit().putBoolean(WifiTriggerManager.KEY_AUTO_ENABLED, enabled).apply()
        if (enabled) {
            WifiTriggerManager.register(activity)
        } else {
            WifiTriggerManager.unregister(activity)
        }
    }

    @JavascriptInterface
    fun openWifiSettings() {
        activity.runOnUiThread {
            val intent = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                Intent(Settings.Panel.ACTION_WIFI)
            } else {
                Intent(Settings.ACTION_WIFI_SETTINGS)
            }
            activity.startActivity(intent)
        }
    }
}
