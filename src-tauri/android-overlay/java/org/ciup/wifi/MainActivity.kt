package org.ciup.wifi

import android.graphics.Color
import android.webkit.WebView
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat

class MainActivity : TauriActivity() {
    private var bridge: WifiBridge? = null

    override fun onWebViewCreate(webView: WebView) {
        super.onWebViewCreate(webView)
        window.decorView.setBackgroundColor(Color.parseColor("#090D16"))
        ViewCompat.setOnApplyWindowInsetsListener(webView) { view, windowInsets ->
            val insets = windowInsets.getInsets(WindowInsetsCompat.Type.systemBars())
            view.setPadding(insets.left, insets.top, insets.right, insets.bottom)
            windowInsets
        }
        val wifiBridge = WifiBridge(this, webView)
        bridge = wifiBridge
        webView.addJavascriptInterface(wifiBridge, "AndroidWifi")

        val prefs = getSharedPreferences(WifiTriggerManager.PREFS_NAME, MODE_PRIVATE)
        val user = prefs.getString(WifiTriggerManager.KEY_USERNAME, "") ?: ""
        val pass = prefs.getString(WifiTriggerManager.KEY_PASSWORD, "") ?: ""
        if (user.isNotEmpty() && pass.isNotEmpty() && prefs.getBoolean(WifiTriggerManager.KEY_AUTO_ENABLED, true)) {
            WifiTriggerManager.register(this)
        }

        wifiBridge.startForegroundCallbacks()
    }

    override fun onStart() {
        super.onStart()
        bridge?.startForegroundCallbacks()
    }

    override fun onStop() {
        bridge?.stopForegroundCallbacks()
        super.onStop()
    }

    override fun onDestroy() {
        bridge?.dispose()
        bridge = null
        super.onDestroy()
    }
}
