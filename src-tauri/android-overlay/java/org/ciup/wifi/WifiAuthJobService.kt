package org.ciup.wifi

import android.app.job.JobParameters
import android.app.job.JobService
import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.os.Build
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
                val targetNet: Network? = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                    params.network ?: findCaptiveWifiNetwork(cm)
                } else {
                    findCaptiveWifiNetwork(cm)
                }
                if (targetNet != null) {
                    WifiTriggerManager.handleCaptiveNetwork(appCtx, targetNet, forceUserInitiated = false)
                }
            } finally {
                jobFinished(params, false)
                WifiTriggerManager.scheduleCaptiveJob(appCtx, delayMs = 15_000L)
            }
        }
        return true
    }

    override fun onStopJob(params: JobParameters): Boolean = false

    private fun findCaptiveWifiNetwork(cm: ConnectivityManager): Network? {
        return try {
            @Suppress("DEPRECATION")
            cm.allNetworks.firstOrNull { net ->
                val caps = cm.getNetworkCapabilities(net)
                caps?.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) == true &&
                    caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_CAPTIVE_PORTAL)
            }
        } catch (_: Exception) {
            null
        }
    }
}
