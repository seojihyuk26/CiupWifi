package org.ciup.wifi

import android.webkit.WebView

class MainActivity : TauriActivity() {
    private var webViewRef: WebView? = null
    private var bridge: WifiBridge? = null

    override fun onWebViewCreate(webView: WebView) {
        super.onWebViewCreate(webView)
        webViewRef = webView
        val wifiBridge = WifiBridge(this, webView)
        bridge = wifiBridge
        webView.addJavascriptInterface(wifiBridge, "AndroidWifi")

        WifiMonitorService.startMonitorIfConfigured(this)
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        WifiMonitorService.refreshState()
        webViewRef?.post {
            webViewRef?.evaluateJavascript(
                "window.__onAndroidPermissionsChanged && window.__onAndroidPermissionsChanged();",
                null
            )
        }
    }

    override fun onDestroy() {
        bridge?.dispose()
        bridge = null
        webViewRef = null
        super.onDestroy()
    }
}
