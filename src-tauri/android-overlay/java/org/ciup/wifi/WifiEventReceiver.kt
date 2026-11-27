package org.ciup.wifi

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.os.Build
import java.util.concurrent.Executors

class WifiEventReceiver : BroadcastReceiver() {

    companion object {
        private val executor = Executors.newSingleThreadExecutor()
    }

    override fun onReceive(context: Context, intent: Intent) {
        val appCtx = context.applicationContext
        val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager

        var network: Network? = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            intent.getParcelableExtra(ConnectivityManager.EXTRA_NETWORK, Network::class.java)
        } else {
            @Suppress("DEPRECATION")
            intent.getParcelableExtra(ConnectivityManager.EXTRA_NETWORK)
        }

        if (network == null) {
            @Suppress("DEPRECATION")
            network = try {
                cm.allNetworks.firstOrNull { net ->
                    cm.getNetworkCapabilities(net)?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true
                }
            } catch (_: Exception) {
                null
            }
        }

        if (network == null) return

        val isCaptiveEvent = intent.action == WifiTriggerManager.ACTION_CAPTIVE_EVENT
        val targetNetwork = network
        val pendingResult = goAsync()
        executor.execute {
            try {
                WifiTriggerManager.handleWifiEvent(appCtx, targetNetwork, isCaptiveEvent)
            } finally {
                pendingResult.finish()
            }
        }
    }
}
