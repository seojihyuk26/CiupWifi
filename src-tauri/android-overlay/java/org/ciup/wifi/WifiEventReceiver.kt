package org.ciup.wifi

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.net.ConnectivityManager
import android.net.Network
import android.os.Build
import java.util.concurrent.Executors

class WifiEventReceiver : BroadcastReceiver() {

    companion object {
        private val executor = Executors.newSingleThreadExecutor()
    }

    override fun onReceive(context: Context, intent: Intent) {
        val network: Network? = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            intent.getParcelableExtra(ConnectivityManager.EXTRA_NETWORK, Network::class.java)
        } else {
            @Suppress("DEPRECATION")
            intent.getParcelableExtra(ConnectivityManager.EXTRA_NETWORK)
        }

        if (network == null) return

        val pendingResult = goAsync()
        val appCtx = context.applicationContext
        executor.execute {
            try {
                WifiTriggerManager.handleCaptiveNetwork(appCtx, network, forceUserInitiated = false)
            } finally {
                pendingResult.finish()
            }
        }
    }
}
