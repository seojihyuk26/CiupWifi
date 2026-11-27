package org.ciup.wifi

import android.app.job.JobParameters
import android.app.job.JobService
import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import java.util.concurrent.Executors

class WifiAuthJobService : JobService() {

    companion object {
        private val executor = Executors.newSingleThreadExecutor()
    }

    override fun onStartJob(params: JobParameters): Boolean {
        val appCtx = applicationContext
        executor.execute {
            try {
                val cm = appCtx.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
                val targetNet = findWifiNetwork(cm)
                if (targetNet != null) {
                    WifiTriggerManager.handleCaptiveNetwork(appCtx, targetNet, forceUserInitiated = false)
                }
            } finally {
                jobFinished(params, false)
            }
        }
        return true
    }

    override fun onStopJob(params: JobParameters): Boolean = false

    private fun findWifiNetwork(cm: ConnectivityManager): Network? {
        return try {
            @Suppress("DEPRECATION")
            val wifiNets = cm.allNetworks.filter { net ->
                cm.getNetworkCapabilities(net)?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true
            }
            wifiNets.firstOrNull { net ->
                val caps = cm.getNetworkCapabilities(net)
                caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL) == true ||
                    caps?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) != true
            } ?: wifiNets.firstOrNull()
        } catch (_: Exception) {
            null
        }
    }
}
