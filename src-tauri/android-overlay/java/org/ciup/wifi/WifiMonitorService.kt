package org.ciup.wifi

import android.Manifest
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.content.pm.ServiceInfo
import android.net.ConnectivityManager
import android.net.LinkProperties
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import android.net.wifi.WifiInfo
import android.net.wifi.WifiManager
import android.os.Build
import android.os.IBinder
import org.json.JSONObject
import java.net.Inet4Address
import java.util.Locale
import java.util.concurrent.Executors
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.TimeUnit

class WifiMonitorService : Service() {

    companion object {
        private const val CHANNEL_ID = "ciupwifi_monitor"
        private const val NOTIF_ID = 1001
        const val PREFS_NAME = "ciupwifi_prefs"
        const val KEY_USERNAME = "username"
        const val KEY_PASSWORD = "password"
        const val KEY_AUTO_ENABLED = "auto_enabled"
        const val KEY_USER_PAUSED = "user_paused"
        const val KEY_AUTH_FAILED = "auth_failed"
        const val KEY_LAST_PORTAL = "last_portal_url"
        const val KEY_LAST_LOGOUT = "last_logout_url"
        const val KEY_LAST_ERROR = "last_error"

        @Volatile
        private var instance: WifiMonitorService? = null

        @Volatile
        private var statusListener: ((String) -> Unit)? = null

        @Volatile
        private var lastSnapshotJson: String = JSONObject().apply {
            put("state", "disconnected")
            put("ssid", JSONObject.NULL)
            put("is_wifi", false)
            put("is_cellular", false)
            put("is_wificity", false)
            put("user_paused", false)
            put("auto_enabled", true)
            put("auth_failed", false)
            put("portal_url", JSONObject.NULL)
            put("detail", "Service initializing...")
        }.toString()

        fun setStatusListener(listener: ((String) -> Unit)?) {
            statusListener = listener
            if (listener != null) {
                listener(getCurrentStatusJson(null))
            }
        }

        fun getCurrentStatusJson(context: Context?): String {
            instance?.let { svc ->
                return svc.buildStatusJson()
            }
            if (context != null) {
                return buildFallbackStatus(context)
            }
            return lastSnapshotJson
        }

        private fun buildFallbackStatus(context: Context): String {
            val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            val cm = context.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
            val activeNet = cm.activeNetwork
            val activeCaps = if (activeNet != null) cm.getNetworkCapabilities(activeNet) else null
            val isWifi = activeCaps?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true
            val isCellular = activeCaps?.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) == true

            val wm = context.applicationContext.getSystemService(Context.WIFI_SERVICE) as? WifiManager
            val ssid = extractSsid(context, activeCaps, wm)
            val isWifiCity = ssid?.lowercase(Locale.ROOT)?.contains("wificity") == true

            val state = when {
                isWifiCity && prefs.getBoolean(KEY_USER_PAUSED, false) -> "paused"
                isWifiCity && activeCaps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) == true &&
                    activeCaps.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) != true -> "wifi_city_online"
                isWifiCity -> "wifi_city_captive"
                isWifi && ssid == null -> "permission_needed"
                isWifi -> "not_on_wificity"
                isCellular -> "not_on_wificity"
                else -> "disconnected"
            }

            return JSONObject().apply {
                put("state", state)
                put("ssid", ssid ?: JSONObject.NULL)
                put("is_wifi", isWifi)
                put("is_cellular", isCellular)
                put("is_wificity", isWifiCity)
                put("user_paused", prefs.getBoolean(KEY_USER_PAUSED, false))
                put("auto_enabled", prefs.getBoolean(KEY_AUTO_ENABLED, true))
                put("auth_failed", prefs.getBoolean(KEY_AUTH_FAILED, false))
                put("portal_url", prefs.getString(KEY_LAST_PORTAL, null) ?: JSONObject.NULL)
                put("detail", prefs.getString(KEY_LAST_ERROR, null) ?: JSONObject.NULL)
            }.toString()
        }

        fun extractSsid(context: Context, caps: NetworkCapabilities?, wm: WifiManager?): String? {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
                val info = caps?.transportInfo as? WifiInfo
                val raw = info?.ssid
                val cleaned = cleanSsid(raw)
                if (cleaned != null) return cleaned
            }
            val hasLoc = context.checkSelfPermission(Manifest.permission.ACCESS_FINE_LOCATION) == PackageManager.PERMISSION_GRANTED ||
                context.checkSelfPermission(Manifest.permission.ACCESS_COARSE_LOCATION) == PackageManager.PERMISSION_GRANTED
            if (hasLoc) {
                @Suppress("DEPRECATION")
                val raw = wm?.connectionInfo?.ssid
                val cleaned = cleanSsid(raw)
                if (cleaned != null) return cleaned
            }
            return null
        }

        private fun cleanSsid(raw: String?): String? {
            if (raw.isNullOrBlank()) return null
            val trimmed = raw.trim().trim('"')
            if (trimmed.isEmpty() || trimmed.equals("<unknown ssid>", ignoreCase = true)) {
                return null
            }
            return trimmed
        }

        fun startMonitorIfConfigured(context: Context) {
            val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            val auto = prefs.getBoolean(KEY_AUTO_ENABLED, true)
            if (!auto) return
            val intent = Intent(context, WifiMonitorService::class.java)
            try {
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    context.startForegroundService(intent)
                } else {
                    context.startService(intent)
                }
            } catch (_: Exception) {
                // Ignored if OS restricts background FGS launch
            }
        }

        fun stopMonitor(context: Context) {
            val intent = Intent(context, WifiMonitorService::class.java)
            context.stopService(intent)
        }

        fun triggerConnectNow(callback: (String) -> Unit) {
            val svc = instance
            if (svc == null) {
                callback(JSONObject().apply {
                    put("ok", false)
                    put("message", "Monitor service not active yet.")
                }.toString())
                return
            }
            svc.connectNowAsync(callback)
        }

        fun triggerDisconnectNow(callback: (String) -> Unit) {
            val svc = instance
            if (svc == null) {
                callback(JSONObject().apply { put("ok", true) }.toString())
                return
            }
            svc.disconnectNowAsync(callback)
        }

        fun refreshState() {
            instance?.refreshEvaluationAsync()
        }
    }

    private lateinit var connectivityManager: ConnectivityManager
    private var wifiManager: WifiManager? = null
    private val executor = Executors.newSingleThreadScheduledExecutor()
    private var watchdogFuture: ScheduledFuture<*>? = null

    private var wifiCallback: ConnectivityManager.NetworkCallback? = null
    private var defaultCallback: ConnectivityManager.NetworkCallback? = null

    @Volatile
    private var currentWifiNetwork: Network? = null

    @Volatile
    private var currentWifiCaps: NetworkCapabilities? = null

    @Volatile
    private var currentLinkProps: LinkProperties? = null

    @Volatile
    private var currentSsid: String? = null

    @Volatile
    private var currentGatewayHints: List<String> = emptyList()

    @Volatile
    private var isWifiCity: Boolean = false

    @Volatile
    private var isWifiCityOnline: Boolean = false

    @Volatile
    private var isCellularDefault: Boolean = false

    @Volatile
    private var needsPermission: Boolean = false

    @Volatile
    private var authInProgress: Boolean = false

    @Volatile
    private var consecutiveFailures: Int = 0

    override fun onCreate() {
        super.onCreate()
        instance = this
        connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        wifiManager = applicationContext.getSystemService(Context.WIFI_SERVICE) as? WifiManager

        createNotificationChannel()
        startForegroundNotification("Waiting for WifiCity")

        registerCallbacks()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForegroundNotification(currentNotificationTitle())
        refreshEvaluationAsync()
        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onDestroy() {
        cancelWatchdog()
        try {
            wifiCallback?.let { connectivityManager.unregisterNetworkCallback(it) }
        } catch (_: Exception) {
        }
        try {
            defaultCallback?.let { connectivityManager.unregisterNetworkCallback(it) }
        } catch (_: Exception) {
        }
        executor.shutdownNow()
        if (instance === this) {
            instance = null
        }
        super.onDestroy()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "WifiCity Auto-Login",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Keeps CIUP WifiCity authenticated automatically"
                setShowBadge(false)
            }
            val nm = getSystemService(NotificationManager::class.java)
            nm?.createNotificationChannel(channel)
        }
    }

    private fun buildNotification(text: String): Notification {
        val openIntent = packageManager.getLaunchIntentForPackage(packageName) ?: Intent(this, MainActivity::class.java)
        val pi = PendingIntent.getActivity(
            this,
            0,
            openIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ID)
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
        }

        return builder
            .setContentTitle("CiupWifi")
            .setContentText(text)
            .setSmallIcon(android.R.drawable.stat_sys_warning)
            .setOngoing(true)
            .setOnlyAlertOnce(true)
            .setContentIntent(pi)
            .build()
    }

    private fun startForegroundNotification(text: String) {
        val notif = buildNotification(text)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(NOTIF_ID, notif, ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE)
        } else {
            startForeground(NOTIF_ID, notif)
        }
    }

    private fun updateNotification(text: String) {
        val nm = getSystemService(Context.NOTIFICATION_SERVICE) as? NotificationManager ?: return
        nm.notify(NOTIF_ID, buildNotification(text))
    }

    private fun currentNotificationTitle(): String {
        val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val paused = prefs.getBoolean(KEY_USER_PAUSED, false)
        return when {
            isWifiCity && paused -> "WifiCity paused"
            isWifiCity && isWifiCityOnline -> "Connected to WifiCity"
            isWifiCity -> "Authenticating WifiCity..."
            currentWifiNetwork != null -> "On ${currentSsid ?: "other Wi-Fi"} (idle)"
            isCellularDefault -> "On mobile data (waiting for WifiCity)"
            else -> "Waiting for WifiCity"
        }
    }

    private fun registerCallbacks() {
        val wifiRequest = NetworkRequest.Builder()
            .addTransportType(NetworkCapabilities.TRANSPORT_WIFI)
            .build()

        val cb = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            object : ConnectivityManager.NetworkCallback(FLAG_INCLUDE_LOCATION_INFO) {
                override fun onAvailable(network: Network) = handleWifiAvailable(network)
                override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) =
                    handleWifiCapabilitiesChanged(network, caps)
                override fun onLinkPropertiesChanged(network: Network, linkProperties: LinkProperties) =
                    handleWifiLinkPropertiesChanged(network, linkProperties)
                override fun onLost(network: Network) = handleWifiLost(network)
            }
        } else {
            object : ConnectivityManager.NetworkCallback() {
                override fun onAvailable(network: Network) = handleWifiAvailable(network)
                override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) =
                    handleWifiCapabilitiesChanged(network, caps)
                override fun onLinkPropertiesChanged(network: Network, linkProperties: LinkProperties) =
                    handleWifiLinkPropertiesChanged(network, linkProperties)
                override fun onLost(network: Network) = handleWifiLost(network)
            }
        }
        wifiCallback = cb
        connectivityManager.registerNetworkCallback(wifiRequest, cb)

        val defCb = object : ConnectivityManager.NetworkCallback() {
            override fun onAvailable(network: Network) {
                val caps = connectivityManager.getNetworkCapabilities(network)
                isCellularDefault = caps?.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) == true &&
                    caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) != true
                emitStatusUpdate()
            }

            override fun onCapabilitiesChanged(network: Network, caps: NetworkCapabilities) {
                isCellularDefault = caps.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR) &&
                    !caps.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)
                emitStatusUpdate()
            }

            override fun onLost(network: Network) {
                isCellularDefault = false
                emitStatusUpdate()
            }
        }
        defaultCallback = defCb
        connectivityManager.registerDefaultNetworkCallback(defCb)
    }

    private fun handleWifiAvailable(network: Network) {
        executor.execute {
            val prevNet = currentWifiNetwork
            currentWifiNetwork = network
            currentWifiCaps = connectivityManager.getNetworkCapabilities(network)
            currentLinkProps = connectivityManager.getLinkProperties(network)
            if (prevNet == null || prevNet != network) {
                clearPausedAndAuthError()
                consecutiveFailures = 0
                isWifiCityOnline = false
            }
            evaluateWifiNetwork(triggerAuthIfCaptive = true)
        }
    }

    private fun handleWifiCapabilitiesChanged(network: Network, caps: NetworkCapabilities) {
        executor.execute {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return@execute
            currentWifiNetwork = network
            currentWifiCaps = caps
            evaluateWifiNetwork(triggerAuthIfCaptive = true)
        }
    }

    private fun handleWifiLinkPropertiesChanged(network: Network, lp: LinkProperties) {
        executor.execute {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return@execute
            currentWifiNetwork = network
            currentLinkProps = lp
            evaluateWifiNetwork(triggerAuthIfCaptive = false)
        }
    }

    private fun handleWifiLost(network: Network) {
        executor.execute {
            if (currentWifiNetwork != null && currentWifiNetwork != network) return@execute
            cancelWatchdog()
            currentWifiNetwork = null
            currentWifiCaps = null
            currentLinkProps = null
            currentSsid = null
            currentGatewayHints = emptyList()
            isWifiCity = false
            isWifiCityOnline = false
            needsPermission = false
            consecutiveFailures = 0
            clearPausedAndAuthError()
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
        }
    }

    private fun clearPausedAndAuthError() {
        getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE).edit()
            .putBoolean(KEY_USER_PAUSED, false)
            .putBoolean(KEY_AUTH_FAILED, false)
            .remove(KEY_LAST_ERROR)
            .apply()
    }

    private fun refreshEvaluationAsync() {
        executor.execute {
            evaluateWifiNetwork(triggerAuthIfCaptive = true)
        }
    }

    private fun evaluateWifiNetwork(triggerAuthIfCaptive: Boolean) {
        val net = currentWifiNetwork
        if (net == null) {
            isWifiCity = false
            isWifiCityOnline = false
            needsPermission = false
            cancelWatchdog()
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
            return
        }

        val caps = currentWifiCaps ?: connectivityManager.getNetworkCapabilities(net)
        val lp = currentLinkProps ?: connectivityManager.getLinkProperties(net)

        val gateways = lp?.routes
            ?.mapNotNull { it.gateway as? Inet4Address }
            ?.mapNotNull { it.hostAddress }
            ?.distinct()
            ?: emptyList()
        currentGatewayHints = gateways

        val ssid = extractSsid(this, caps, wifiManager)
        currentSsid = ssid

        val wasWifiCity = isWifiCity
        var detectedWifiCity = false
        var permNeeded = false

        if (ssid != null) {
            detectedWifiCity = ssid.lowercase(Locale.ROOT).contains("wificity")
        } else {
            // SSID hidden because Location permission or GPS is off -> check CIUP 10.254.x.x gateway or captive probe over wifi Network
            val hasCiupGateway = gateways.any { it.startsWith("10.254.") }
            if (hasCiupGateway) {
                detectedWifiCity = true
            } else if (caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) == true) {
                val probe = WifiPortalClient.probeConnectivity(net, gateways)
                if (probe is WifiPortalClient.ProbeResult.Captive &&
                    (probe.portalUrl.contains("10.254.") || probe.portalUrl.contains("fgtauth"))
                ) {
                    detectedWifiCity = true
                } else {
                    permNeeded = true
                }
            } else {
                permNeeded = true
            }
        }

        isWifiCity = detectedWifiCity
        needsPermission = permNeeded

        if (!wasWifiCity && isWifiCity) {
            clearPausedAndAuthError()
            consecutiveFailures = 0
        }

        if (!isWifiCity) {
            isWifiCityOnline = false
            cancelWatchdog()
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
            return
        }

        val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val paused = prefs.getBoolean(KEY_USER_PAUSED, false)
        val authFailed = prefs.getBoolean(KEY_AUTH_FAILED, false)

        if (paused) {
            isWifiCityOnline = false
            cancelWatchdog()
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
            return
        }

        val hasCaptiveCap = caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) == true
        val isValidated = caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) == true

        if (isValidated && !hasCaptiveCap) {
            isWifiCityOnline = true
            consecutiveFailures = 0
            ensureWatchdogRunning()
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
            return
        }

        // Needs check / authentication on WifiCity
        isWifiCityOnline = false
        ensureWatchdogRunning()
        updateNotification(currentNotificationTitle())
        emitStatusUpdate()

        if (triggerAuthIfCaptive && !authFailed) {
            authenticateOnWifiNetwork(net, gateways)
        }
    }

    private fun authenticateOnWifiNetwork(net: Network, gateways: List<String>) {
        if (authInProgress) return
        val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val user = prefs.getString(KEY_USERNAME, "") ?: ""
        val pass = prefs.getString(KEY_PASSWORD, "") ?: ""
        if (user.isEmpty() || pass.isEmpty()) return
        if (prefs.getBoolean(KEY_USER_PAUSED, false) || prefs.getBoolean(KEY_AUTH_FAILED, false)) return

        authInProgress = true
        try {
            val probe = WifiPortalClient.probeConnectivity(net, gateways)
            if (probe is WifiPortalClient.ProbeResult.Online) {
                isWifiCityOnline = true
                consecutiveFailures = 0
                connectivityManager.reportNetworkConnectivity(net, true)
                prefs.edit().putBoolean(KEY_AUTH_FAILED, false).remove(KEY_LAST_ERROR).apply()
                updateNotification(currentNotificationTitle())
                emitStatusUpdate()
                return
            }

            val res = WifiPortalClient.login(net, user, pass, gateways)
            if (res.ok) {
                isWifiCityOnline = true
                consecutiveFailures = 0
                connectivityManager.reportNetworkConnectivity(net, true)
                prefs.edit().apply {
                    putBoolean(KEY_AUTH_FAILED, false)
                    remove(KEY_LAST_ERROR)
                    if (res.portalUrl != null) putString(KEY_LAST_PORTAL, res.portalUrl)
                    if (res.logoutUrl != null) putString(KEY_LAST_LOGOUT, res.logoutUrl)
                }.apply()
            } else if (res.authFailed) {
                isWifiCityOnline = false
                prefs.edit()
                    .putBoolean(KEY_AUTH_FAILED, true)
                    .putString(KEY_LAST_ERROR, res.message)
                    .apply()
            } else {
                isWifiCityOnline = false
                consecutiveFailures += 1
                prefs.edit().putString(KEY_LAST_ERROR, res.message).apply()
                val delaySec = listOf(5L, 10L, 20L, 30L)[minOf(consecutiveFailures - 1, 3)]
                executor.schedule({
                    val curNet = currentWifiNetwork
                    if (curNet != null && isWifiCity) {
                        authenticateOnWifiNetwork(curNet, currentGatewayHints)
                    }
                }, delaySec, TimeUnit.SECONDS)
            }
        } finally {
            authInProgress = false
            updateNotification(currentNotificationTitle())
            emitStatusUpdate()
        }
    }

    private fun ensureWatchdogRunning() {
        if (watchdogFuture != null && !watchdogFuture!!.isDone) return
        watchdogFuture = executor.scheduleWithFixedDelay({
            val net = currentWifiNetwork ?: return@scheduleWithFixedDelay
            if (!isWifiCity) return@scheduleWithFixedDelay
            val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            if (prefs.getBoolean(KEY_USER_PAUSED, false) || prefs.getBoolean(KEY_AUTH_FAILED, false)) {
                return@scheduleWithFixedDelay
            }
            when (WifiPortalClient.probeConnectivity(net, currentGatewayHints)) {
                is WifiPortalClient.ProbeResult.Online -> {
                    if (!isWifiCityOnline) {
                        isWifiCityOnline = true
                        consecutiveFailures = 0
                        connectivityManager.reportNetworkConnectivity(net, true)
                        updateNotification(currentNotificationTitle())
                        emitStatusUpdate()
                    }
                }
                is WifiPortalClient.ProbeResult.Captive,
                is WifiPortalClient.ProbeResult.Offline -> {
                    isWifiCityOnline = false
                    emitStatusUpdate()
                    authenticateOnWifiNetwork(net, currentGatewayHints)
                }
            }
        }, 45, 45, TimeUnit.SECONDS)
    }

    private fun cancelWatchdog() {
        watchdogFuture?.cancel(false)
        watchdogFuture = null
    }

    private fun connectNowAsync(callback: (String) -> Unit) {
        executor.execute {
            val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            prefs.edit()
                .putBoolean(KEY_USER_PAUSED, false)
                .putBoolean(KEY_AUTH_FAILED, false)
                .remove(KEY_LAST_ERROR)
                .apply()

            evaluateWifiNetwork(triggerAuthIfCaptive = false)
            val net = currentWifiNetwork
            if (net == null || !isWifiCity) {
                val msg = when {
                    needsPermission -> "Grant Location permission so CiupWifi can verify the WifiCity network."
                    currentSsid != null -> "Connected to '$currentSsid' (not WifiCity). Switch to WifiCity first."
                    else -> "Not connected to the WifiCity Wi-Fi network."
                }
                emitStatusUpdate()
                callback(JSONObject().apply {
                    put("ok", false)
                    put("auth_failed", false)
                    put("message", msg)
                }.toString())
                return@execute
            }

            val user = prefs.getString(KEY_USERNAME, "") ?: ""
            val pass = prefs.getString(KEY_PASSWORD, "") ?: ""
            if (user.isEmpty() || pass.isEmpty()) {
                callback(JSONObject().apply {
                    put("ok", false)
                    put("auth_failed", false)
                    put("message", "Please enter username and password.")
                }.toString())
                return@execute
            }

            val res = WifiPortalClient.login(net, user, pass, currentGatewayHints)
            if (res.ok) {
                isWifiCityOnline = true
                consecutiveFailures = 0
                connectivityManager.reportNetworkConnectivity(net, true)
                prefs.edit().apply {
                    putBoolean(KEY_AUTH_FAILED, false)
                    remove(KEY_LAST_ERROR)
                    if (res.portalUrl != null) putString(KEY_LAST_PORTAL, res.portalUrl)
                    if (res.logoutUrl != null) putString(KEY_LAST_LOGOUT, res.logoutUrl)
                }.apply()
                ensureWatchdogRunning()
            } else if (res.authFailed) {
                isWifiCityOnline = false
                prefs.edit()
                    .putBoolean(KEY_AUTH_FAILED, true)
                    .putString(KEY_LAST_ERROR, res.message)
                    .apply()
            } else {
                isWifiCityOnline = false
                prefs.edit().putString(KEY_LAST_ERROR, res.message).apply()
            }

            updateNotification(currentNotificationTitle())
            emitStatusUpdate()

            callback(JSONObject().apply {
                put("ok", res.ok)
                put("auth_failed", res.authFailed)
                put("portal_url", res.portalUrl ?: JSONObject.NULL)
                put("message", res.message)
            }.toString())
        }
    }

    private fun disconnectNowAsync(callback: (String) -> Unit) {
        executor.execute {
            val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
            val logoutUrl = prefs.getString(KEY_LAST_LOGOUT, null)
            prefs.edit().putBoolean(KEY_USER_PAUSED, true).apply()

            isWifiCityOnline = false
            cancelWatchdog()

            val net = currentWifiNetwork
            if (net != null && !logoutUrl.isNullOrBlank()) {
                WifiPortalClient.logout(net, logoutUrl)
                connectivityManager.reportNetworkConnectivity(net, false)
            }

            updateNotification(currentNotificationTitle())
            emitStatusUpdate()

            callback(JSONObject().apply { put("ok", true) }.toString())
        }
    }

    private fun buildStatusJson(): String {
        val prefs = getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val paused = prefs.getBoolean(KEY_USER_PAUSED, false)
        val auto = prefs.getBoolean(KEY_AUTO_ENABLED, true)
        val authFailed = prefs.getBoolean(KEY_AUTH_FAILED, false)
        val lastPortal = prefs.getString(KEY_LAST_PORTAL, null)
        val lastError = prefs.getString(KEY_LAST_ERROR, null)
        val hasWifi = currentWifiNetwork != null

        val state = when {
            !hasWifi && isCellularDefault -> "not_on_wificity"
            !hasWifi -> "disconnected"
            needsPermission && !isWifiCity -> "permission_needed"
            !isWifiCity -> "not_on_wificity"
            paused -> "paused"
            isWifiCityOnline -> "wifi_city_online"
            else -> "wifi_city_captive"
        }

        val detail = when (state) {
            "permission_needed" -> "Grant Location permission so CiupWifi can verify the WifiCity network."
            "not_on_wificity" -> when {
                !hasWifi && isCellularDefault -> "On Mobile Data (Not on WifiCity)."
                currentSsid != null -> "Connected to '$currentSsid' (Not WifiCity)."
                else -> "Not connected to WifiCity."
            }
            "paused" -> "Disconnected by user. Tap Connect or rejoin WifiCity to resume."
            "disconnected" -> "Wi-Fi is disconnected."
            else -> lastError
        }

        val json = JSONObject().apply {
            put("state", state)
            put("ssid", currentSsid ?: JSONObject.NULL)
            put("is_wifi", hasWifi)
            put("is_cellular", !hasWifi && isCellularDefault)
            put("is_wificity", isWifiCity)
            put("user_paused", paused)
            put("auto_enabled", auto)
            put("auth_failed", authFailed)
            put("portal_url", lastPortal ?: JSONObject.NULL)
            put("detail", detail ?: JSONObject.NULL)
        }.toString()
        lastSnapshotJson = json
        return json
    }

    private fun emitStatusUpdate() {
        val json = buildStatusJson()
        statusListener?.invoke(json)
    }
}
