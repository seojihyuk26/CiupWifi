package org.ciup.wifi

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

class BootReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        val action = intent.action ?: return
        if (action == Intent.ACTION_BOOT_COMPLETED || action == Intent.ACTION_MY_PACKAGE_REPLACED) {
            val prefs = context.getSharedPreferences(WifiMonitorService.PREFS_NAME, Context.MODE_PRIVATE)
            val user = prefs.getString(WifiMonitorService.KEY_USERNAME, "") ?: ""
            val pass = prefs.getString(WifiMonitorService.KEY_PASSWORD, "") ?: ""
            val auto = prefs.getBoolean(WifiMonitorService.KEY_AUTO_ENABLED, true)
            if (auto && user.isNotEmpty() && pass.isNotEmpty()) {
                WifiMonitorService.startMonitorIfConfigured(context)
            }
        }
    }
}
