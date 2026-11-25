package org.ciup.wifi

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.location.LocationManager
import android.net.Uri
import android.os.Build
import android.os.PowerManager
import android.provider.Settings
import android.webkit.JavascriptInterface
import android.webkit.WebView
import org.json.JSONObject

class WifiBridge(
    private val activity: Activity,
    private val webView: WebView
) {
    companion object {
        const val REQ_PERMISSIONS = 2601
    }

    init {
        WifiMonitorService.setStatusListener { json ->
            val escaped = JSONObject.quote(json)
            webView.post {
                webView.evaluateJavascript(
                    "window.__onAndroidWifiStatus && window.__onAndroidWifiStatus(JSON.parse($escaped));",
                    null
                )
            }
        }
    }

    fun dispose() {
        WifiMonitorService.setStatusListener(null)
    }

    @JavascriptInterface
    fun getNetworkStatus(): String {
        return WifiMonitorService.getCurrentStatusJson(activity)
    }

    @JavascriptInterface
    fun saveCredentials(username: String, password: String): String {
        val prefs = activity.getSharedPreferences(WifiMonitorService.PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit()
            .putString(WifiMonitorService.KEY_USERNAME, username)
            .putString(WifiMonitorService.KEY_PASSWORD, password)
            .putBoolean(WifiMonitorService.KEY_AUTH_FAILED, false)
            .remove(WifiMonitorService.KEY_LAST_ERROR)
            .apply()
        WifiMonitorService.startMonitorIfConfigured(activity)
        WifiMonitorService.refreshState()
        return JSONObject().apply { put("ok", true) }.toString()
    }

    @JavascriptInterface
    fun loadCredentials(): String {
        val prefs = activity.getSharedPreferences(WifiMonitorService.PREFS_NAME, Context.MODE_PRIVATE)
        return JSONObject().apply {
            put("username", prefs.getString(WifiMonitorService.KEY_USERNAME, "") ?: "")
            put("password", prefs.getString(WifiMonitorService.KEY_PASSWORD, "") ?: "")
        }.toString()
    }

    @JavascriptInterface
    fun connectNow(requestId: String) {
        WifiMonitorService.startMonitorIfConfigured(activity)
        WifiMonitorService.triggerConnectNow { resultJson ->
            val reqQuoted = JSONObject.quote(requestId)
            val resQuoted = JSONObject.quote(resultJson)
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
        WifiMonitorService.triggerDisconnectNow { resultJson ->
            val reqQuoted = JSONObject.quote(requestId)
            val resQuoted = JSONObject.quote(resultJson)
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
        val prefs = activity.getSharedPreferences(WifiMonitorService.PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit().putBoolean(WifiMonitorService.KEY_AUTO_ENABLED, enabled).apply()
        if (enabled) {
            WifiMonitorService.startMonitorIfConfigured(activity)
        } else {
            WifiMonitorService.stopMonitor(activity)
        }
    }

    @JavascriptInterface
    fun getPermissionsStatus(): String {
        val fine = activity.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED
        val coarse = activity.checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
        val notif = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            activity.checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) == PackageManager.PERMISSION_GRANTED
        } else {
            true
        }
        val lm = activity.getSystemService(Context.LOCATION_SERVICE) as? LocationManager
        val locationEnabled = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
            lm?.isLocationEnabled ?: true
        } else {
            true
        }
        val pm = activity.getSystemService(Context.POWER_SERVICE) as? PowerManager
        val batteryIgnored = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M) {
            pm?.isIgnoringBatteryOptimizations(activity.packageName) ?: true
        } else {
            true
        }

        return JSONObject().apply {
            put("locationGranted", fine || coarse)
            put("notificationsGranted", notif)
            put("locationEnabled", locationEnabled)
            put("batteryIgnored", batteryIgnored)
        }.toString()
    }

    @JavascriptInterface
    fun requestPermissions() {
        val toRequest = mutableListOf<String>()
        if (activity.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) != PackageManager.PERMISSION_GRANTED) {
            toRequest.add(Manifest.permission.ACCESS_FINE_LOCATION)
        }
        if (activity.checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) != PackageManager.PERMISSION_GRANTED) {
            toRequest.add(Manifest.permission.ACCESS_COARSE_LOCATION)
        }
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
            activity.checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED
        ) {
            toRequest.add(Manifest.permission.POST_NOTIFICATIONS)
        }
        if (toRequest.isNotEmpty()) {
            activity.runOnUiThread {
                activity.requestPermissions(toRequest.toTypedArray(), REQ_PERMISSIONS)
            }
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

    @JavascriptInterface
    fun requestIgnoreBatteryOptimizations() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.M) return
        val pm = activity.getSystemService(Context.POWER_SERVICE) as? PowerManager
        if (pm?.isIgnoringBatteryOptimizations(activity.packageName) == true) return
        activity.runOnUiThread {
            try {
                val intent = Intent(Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS).apply {
                    data = Uri.parse("package:${activity.packageName}")
                }
                activity.startActivity(intent)
            } catch (_: Exception) {
                val fallback = Intent(Settings.ACTION_IGNORE_BATTERY_OPTIMIZATION_SETTINGS)
                activity.startActivity(fallback)
            }
        }
    }
}
