package org.ciup.wifi

import android.net.Network
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.OutputStreamWriter
import java.net.HttpURLConnection
import java.net.URL
import java.net.URLEncoder
import java.security.SecureRandom
import java.security.cert.X509Certificate
import java.util.Locale
import javax.net.ssl.HostnameVerifier
import javax.net.ssl.HttpsURLConnection
import javax.net.ssl.SSLContext
import javax.net.ssl.TrustManager
import javax.net.ssl.X509TrustManager

object WifiPortalClient {

    sealed class ProbeResult {
        object Online : ProbeResult()
        data class Captive(val portalUrl: String, val initialHtml: String? = null) : ProbeResult()
        object Offline : ProbeResult()
    }

    data class LoginResult(
        val ok: Boolean,
        val authFailed: Boolean = false,
        val portalUrl: String? = null,
        val logoutUrl: String? = null,
        val message: String = ""
    )

    private val portalSslSocketFactory by lazy {
        val trustAll = arrayOf<TrustManager>(object : X509TrustManager {
            override fun checkClientTrusted(chain: Array<out X509Certificate>?, authType: String?) {}
            override fun checkServerTrusted(chain: Array<out X509Certificate>?, authType: String?) {}
            override fun getAcceptedIssuers(): Array<X509Certificate> = emptyArray()
        })
        val ctx = SSLContext.getInstance("TLS")
        ctx.init(null, trustAll, SecureRandom())
        ctx.socketFactory
    }

    private val portalHostnameVerifier = HostnameVerifier { _, _ -> true }

    private fun openBoundConnection(
        network: Network,
        url: URL,
        timeoutMs: Int,
        acceptPortalCerts: Boolean
    ): HttpURLConnection {
        val conn = network.openConnection(url) as HttpURLConnection
        conn.connectTimeout = timeoutMs
        conn.readTimeout = timeoutMs
        conn.instanceFollowRedirects = false
        conn.useCaches = false
        conn.setRequestProperty("Cache-Control", "no-cache")
        if (acceptPortalCerts && conn is HttpsURLConnection) {
            conn.sslSocketFactory = portalSslSocketFactory
            conn.hostnameVerifier = portalHostnameVerifier
        }
        return conn
    }

    private fun readBody(conn: HttpURLConnection): String {
        val stream = try {
            if (conn.responseCode >= 400) conn.errorStream ?: conn.inputStream else conn.inputStream
        } catch (_: Exception) {
            conn.errorStream
        } ?: return ""
        return BufferedReader(InputStreamReader(stream, Charsets.UTF_8)).use { it.readText() }
    }

    private fun resolveUrl(base: URL, target: String): URL? {
        val clean = target.replace("&amp;", "&").trim().trim('"', '\'')
        if (clean.isEmpty() || clean.startsWith("#") || clean.lowercase(Locale.ROOT).startsWith("javascript:")) {
            return null
        }
        return try {
            URL(base, clean)
        } catch (_: Exception) {
            null
        }
    }

    private fun isCloudflareUpgrade(loc: String): Boolean {
        val lower = loc.lowercase(Locale.ROOT)
        return lower.startsWith("https://1.1.1.1") ||
            lower.contains("cloudflare.com") ||
            lower.contains("one.one.one.one")
    }

    fun looksLikeLoginForm(html: String): Boolean {
        val lower = html.lowercase(Locale.ROOT)
        return (lower.contains("<form") && lower.contains("password")) ||
            lower.contains("fgtauth") ||
            lower.contains("ft_un") ||
            lower.contains("ft_pd") ||
            (lower.contains("<form") && lower.contains("magic"))
    }

    private fun extractRedirectFromHtml(html: String, baseUrl: URL): String? {
        val lower = html.lowercase(Locale.ROOT)
        val metaRegex = Regex(
            """http-equiv\s*=\s*["']?refresh["']?[^>]*content\s*=\s*["'][^"']*url\s*=\s*([^"'\s>]+)""",
            RegexOption.IGNORE_CASE
        )
        metaRegex.find(html)?.groupValues?.getOrNull(1)?.let { candidate ->
            resolveUrl(baseUrl, candidate)?.let { return it.toString() }
        }

        for (pat in listOf(
            "location.href=",
            "location.href =",
            "window.location=",
            "window.location =",
            "top.location=",
            "top.location =",
            "location.replace(",
            "location.assign("
        )) {
            val idx = lower.indexOf(pat)
            if (idx >= 0) {
                val rest = html.substring(idx + pat.length).trimStart()
                val q = rest.firstOrNull()
                if (q == '"' || q == '\'') {
                    val end = rest.indexOf(q, startIndex = 1)
                    if (end > 1) {
                        val candidate = rest.substring(1, end)
                        resolveUrl(baseUrl, candidate)?.let { return it.toString() }
                    }
                }
            }
        }
        return null
    }

    private fun extractMagicToken(urlStr: String): String? {
        val mIdx = urlStr.indexOf("magic=")
        if (mIdx >= 0) {
            val tok = urlStr.substring(mIdx + 6).substringBefore('&')
            if (tok.isNotEmpty()) return tok
        }
        val fIdx = urlStr.indexOf("fgtauth?")
        if (fIdx >= 0) {
            val tok = urlStr.substring(fIdx + 8).substringBefore('&')
            if (tok.isNotEmpty() && !tok.contains('=')) return tok
        }
        return null
    }

    private fun getHtmlAttr(tag: String, attrName: String): String? {
        val pattern = Regex(
            """(?:^|\s)$attrName\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+))""",
            RegexOption.IGNORE_CASE
        )
        val m = pattern.find(tag) ?: return null
        val raw = m.groupValues[1].ifEmpty { m.groupValues[2].ifEmpty { m.groupValues[3] } }
        return raw.replace("&amp;", "&")
    }

    private data class ParsedForm(
        val actionUrl: String,
        val fields: LinkedHashMap<String, String>,
        val userField: String?,
        val passField: String?
    )

    private fun parseForm(html: String, baseUrl: URL): ParsedForm {
        val lower = html.lowercase(Locale.ROOT)
        var chosenBlock = html
        var searchPos = 0
        var firstFound = false
        while (true) {
            val fStart = lower.indexOf("<form", searchPos)
            if (fStart < 0) break
            val fEndRaw = lower.indexOf("</form>", fStart)
            val fEnd = if (fEndRaw >= 0) fEndRaw + 7 else html.length
            val blockLower = lower.substring(fStart, fEnd)
            if (!firstFound || blockLower.contains("password") || blockLower.contains("ft_un") || blockLower.contains("ft_pd")) {
                chosenBlock = html.substring(fStart, fEnd)
                firstFound = true
                if (blockLower.contains("password") || blockLower.contains("ft_un")) break
            }
            searchPos = fEnd
        }

        val blockLower = chosenBlock.lowercase(Locale.ROOT)
        var actionUrl = baseUrl.toString()
        val formTagEnd = blockLower.indexOf('>')
        if (formTagEnd > 0) {
            val openTag = chosenBlock.substring(0, formTagEnd)
            getHtmlAttr(openTag, "action")?.let { act ->
                if (act.isNotBlank()) {
                    resolveUrl(baseUrl, act)?.let { actionUrl = it.toString() }
                }
            }
        }

        val fields = LinkedHashMap<String, String>()
        var userField: String? = null
        var passField: String? = null
        var pos = 0

        while (true) {
            val iStart = blockLower.indexOf("<input", pos)
            if (iStart < 0) break
            val iEnd = blockLower.indexOf('>', iStart)
            if (iEnd < 0) break
            val tag = chosenBlock.substring(iStart, iEnd)
            val name = getHtmlAttr(tag, "name")
            val value = getHtmlAttr(tag, "value") ?: ""
            val type = (getHtmlAttr(tag, "type") ?: "text").lowercase(Locale.ROOT)
            val id = (getHtmlAttr(tag, "id") ?: "").lowercase(Locale.ROOT)

            if (!name.isNullOrEmpty()) {
                val nl = name.lowercase(Locale.ROOT)
                if (type != "submit" && type != "button" && type != "reset") {
                    fields[name] = value
                }
                if (type == "password" || id == "ft_pd" || nl.contains("pass") || nl == "ft_pd") {
                    passField = name
                } else if (userField == null && (id == "ft_un" || nl == "ft_un" || nl.contains("user") || nl.contains("login") || type == "text" || type == "email")) {
                    userField = name
                }
            }
            pos = iEnd + 1
        }

        return ParsedForm(actionUrl, fields, userField, passField)
    }

    private fun extractLogoutUrl(html: String, baseUrl: URL): String? {
        val regex = Regex("""["'(]([^"'\s()>]*(?:/logout|logout\?)[^"'\s()>]*)""", RegexOption.IGNORE_CASE)
        val match = regex.find(html)?.groupValues?.getOrNull(1) ?: return null
        return resolveUrl(baseUrl, match)?.toString()
    }

    fun probeConnectivity(network: Network, gatewayHints: List<String> = emptyList()): ProbeResult {
        val endpoints = listOf(
            "http://connectivitycheck.gstatic.com/generate_204",
            "http://captive.apple.com/hotspot-detect.html",
            "http://detectportal.firefox.com/canonical.html",
            "http://www.msftconnecttest.com/connecttest.txt",
            "http://neverssl.com/",
            "http://1.1.1.1/"
        )

        for (endpoint in endpoints) {
            var conn: HttpURLConnection? = null
            try {
                val url = URL(endpoint)
                conn = openBoundConnection(network, url, timeoutMs = 4000, acceptPortalCerts = true)
                val code = conn.responseCode

                if (code == 204 && endpoint.contains("generate_204")) {
                    return ProbeResult.Online
                }

                val loc = conn.getHeaderField("Location")
                if (!loc.isNullOrBlank()) {
                    if (endpoint == "http://1.1.1.1/" && isCloudflareUpgrade(loc)) {
                        return ProbeResult.Online
                    }
                    resolveUrl(url, loc)?.let { resolved ->
                        return ProbeResult.Captive(resolved.toString())
                    }
                }

                if (code == 200) {
                    val body = readBody(conn)
                    val lower = body.lowercase(Locale.ROOT)
                    if (endpoint.contains("captive.apple.com") &&
                        body.contains("Success") &&
                        !looksLikeLoginForm(body) &&
                        !lower.contains("http-equiv")
                    ) {
                        return ProbeResult.Online
                    }
                    if (endpoint.contains("detectportal.firefox.com") &&
                        lower.contains("success") &&
                        !looksLikeLoginForm(body) &&
                        !lower.contains("http-equiv")
                    ) {
                        return ProbeResult.Online
                    }
                    if (endpoint.contains("msftconnecttest.com") &&
                        body.trim() == "Microsoft Connect Test"
                    ) {
                        return ProbeResult.Online
                    }
                    extractRedirectFromHtml(body, url)?.let { redir ->
                        return ProbeResult.Captive(redir)
                    }
                    if (looksLikeLoginForm(body)) {
                        return ProbeResult.Captive(url.toString(), body)
                    }
                }
            } catch (_: Exception) {
                // try next probe endpoint
            } finally {
                conn?.disconnect()
            }
        }

        // Dynamic gateway fallback across common ports/schemes
        val hosts = LinkedHashSet<String>()
        gatewayHints.map { it.trim() }.filter { it.isNotEmpty() }.forEach { hosts.add(it) }
        hosts.add("10.254.0.254")

        val portSchemes = listOf(
            "http" to "",
            "http" to ":1000",
            "https" to ":1003",
            "http" to ":8000",
            "http" to ":8080",
            "https" to ""
        )

        for (host in hosts) {
            for ((scheme, port) in portSchemes) {
                var conn: HttpURLConnection? = null
                try {
                    val url = URL("$scheme://$host$port/")
                    conn = openBoundConnection(network, url, timeoutMs = 2500, acceptPortalCerts = true)
                    val code = conn.responseCode
                    val loc = conn.getHeaderField("Location")
                    if (!loc.isNullOrBlank()) {
                        resolveUrl(url, loc)?.let { resolved ->
                            return ProbeResult.Captive(resolved.toString())
                        }
                    }
                    if (code in 200..299) {
                        val body = readBody(conn)
                        extractRedirectFromHtml(body, url)?.let { redir ->
                            return ProbeResult.Captive(redir)
                        }
                        if (looksLikeLoginForm(body)) {
                            return ProbeResult.Captive(url.toString(), body)
                        }
                    }
                } catch (_: Exception) {
                    // try next port/scheme
                } finally {
                    conn?.disconnect()
                }
            }
        }

        return ProbeResult.Offline
    }

    private fun storeCookies(conn: HttpURLConnection, cookies: MutableMap<String, String>) {
        val headerValues = conn.headerFields["Set-Cookie"] ?: conn.headerFields["set-cookie"] ?: return
        for (raw in headerValues) {
            val kv = raw.substringBefore(';').trim()
            val eq = kv.indexOf('=')
            if (eq > 0) {
                cookies[kv.substring(0, eq).trim()] = kv.substring(eq + 1).trim()
            }
        }
    }

    private fun applyCookies(conn: HttpURLConnection, cookies: Map<String, String>) {
        if (cookies.isNotEmpty()) {
            conn.setRequestProperty("Cookie", cookies.entries.joinToString("; ") { "${it.key}=${it.value}" })
        }
    }

    fun login(
        network: Network,
        username: String,
        password: String,
        gatewayHints: List<String> = emptyList()
    ): LoginResult {
        val probe = probeConnectivity(network, gatewayHints)
        val (initialUrl, initialHtml) = when (probe) {
            is ProbeResult.Online -> return LoginResult(ok = true, message = "Already connected to WifiCity.")
            is ProbeResult.Offline -> return LoginResult(ok = false, message = "Captive portal not detected on WifiCity.")
            is ProbeResult.Captive -> probe.portalUrl to probe.initialHtml
        }

        var currentUrl = try {
            URL(initialUrl)
        } catch (e: Exception) {
            return LoginResult(ok = false, message = "Invalid portal URL: ${e.message}")
        }

        val cookies = LinkedHashMap<String, String>()
        var magicToken = extractMagicToken(initialUrl)
        var htmlText = initialHtml ?: ""

        if (initialHtml == null) {
            for (hop in 0 until 6) {
                var conn: HttpURLConnection? = null
                try {
                    conn = openBoundConnection(network, currentUrl, timeoutMs = 10000, acceptPortalCerts = true)
                    applyCookies(conn, cookies)
                    val code = conn.responseCode
                    storeCookies(conn, cookies)
                    if (magicToken == null) {
                        magicToken = extractMagicToken(currentUrl.toString())
                    }

                    val loc = conn.getHeaderField("Location")
                    if (code in 300..399 && !loc.isNullOrBlank()) {
                        val nextUrl = resolveUrl(currentUrl, loc)
                        if (nextUrl != null) {
                            currentUrl = nextUrl
                            continue
                        }
                    }

                    htmlText = readBody(conn)
                    if (looksLikeLoginForm(htmlText)) {
                        break
                    }
                    val nextHtmlRedir = extractRedirectFromHtml(htmlText, currentUrl)
                    if (nextHtmlRedir != null) {
                        val nextUrl = URL(nextHtmlRedir)
                        if (nextUrl.toString() != currentUrl.toString()) {
                            currentUrl = nextUrl
                            continue
                        }
                    }
                    break
                } catch (e: Exception) {
                    return LoginResult(ok = false, message = "Failed to connect to captive portal: ${e.message}")
                } finally {
                    conn?.disconnect()
                }
            }
        }

        if (magicToken == null) {
            magicToken = extractMagicToken(currentUrl.toString())
        }

        val parsed = parseForm(htmlText, currentUrl)
        val fields = parsed.fields
        if (!magicToken.isNullOrEmpty() && fields["magic"].isNullOrEmpty()) {
            fields["magic"] = magicToken
        }

        if (parsed.userField != null) {
            fields[parsed.userField] = username
        }
        if (parsed.passField != null) {
            fields[parsed.passField] = password
        }
        if (parsed.userField == null && parsed.passField == null) {
            fields["ft_un"] = username
            fields["ft_pd"] = password
            fields["username"] = username
            fields["password"] = password
        } else {
            if (fields.containsKey("ft_un")) fields["ft_un"] = username
            if (fields.containsKey("ft_pd")) fields["ft_pd"] = password
        }

        val encodedBody = fields.entries.joinToString("&") { (k, v) ->
            "${URLEncoder.encode(k, "UTF-8")}=${URLEncoder.encode(v, "UTF-8")}"
        }

        var postUrl = try {
            URL(parsed.actionUrl)
        } catch (_: Exception) {
            currentUrl
        }
        var postBody = ""
        var finalPostUrl = postUrl

        try {
            var methodPost = true
            var referer = currentUrl.toString()
            for (hop in 0 until 5) {
                var conn: HttpURLConnection? = null
                try {
                    conn = openBoundConnection(network, postUrl, timeoutMs = 10000, acceptPortalCerts = true)
                    applyCookies(conn, cookies)
                    conn.setRequestProperty("Referer", referer)
                    if (methodPost) {
                        conn.requestMethod = "POST"
                        conn.doOutput = true
                        conn.setRequestProperty("Content-Type", "application/x-www-form-urlencoded")
                        OutputStreamWriter(conn.outputStream, Charsets.UTF_8).use { it.write(encodedBody) }
                    }
                    val code = conn.responseCode
                    storeCookies(conn, cookies)
                    finalPostUrl = postUrl

                    val loc = conn.getHeaderField("Location")
                    if (code in 300..399 && !loc.isNullOrBlank()) {
                        val nextUrl = resolveUrl(postUrl, loc)
                        if (nextUrl != null) {
                            referer = postUrl.toString()
                            postUrl = nextUrl
                            methodPost = false
                            continue
                        }
                    }

                    postBody = readBody(conn)
                    break
                } finally {
                    conn?.disconnect()
                }
            }
        } catch (e: Exception) {
            return LoginResult(ok = false, portalUrl = currentUrl.toString(), message = "Login request failed: ${e.message}")
        }

        var logoutUrl = extractLogoutUrl(postBody, finalPostUrl)
        if (logoutUrl == null && !magicToken.isNullOrEmpty() && finalPostUrl.toString().lowercase(Locale.ROOT).contains("keepalive")) {
            logoutUrl = resolveUrl(currentUrl, "/logout?$magicToken")?.toString()
        }

        val lowerBody = postBody.lowercase(Locale.ROOT)
        val explicitAuthFail = lowerBody.contains("authentication failed") ||
            lowerBody.contains("invalid username") ||
            lowerBody.contains("invalid password") ||
            lowerBody.contains("wrong password") ||
            lowerBody.contains("login failed") ||
            lowerBody.contains("failed to authenticate") ||
            lowerBody.contains("incorrect username") ||
            lowerBody.contains("incorrect password")

        val explicitSuccessHint = lowerBody.contains("keepalive") ||
            lowerBody.contains("keep this window open") ||
            lowerBody.contains("authentication succeeded") ||
            lowerBody.contains("welcome") ||
            postBody.contains("Success")

        try {
            Thread.sleep(700)
        } catch (_: InterruptedException) {
        }

        if (probeConnectivity(network, gatewayHints) is ProbeResult.Online) {
            return LoginResult(
                ok = true,
                portalUrl = currentUrl.toString(),
                logoutUrl = logoutUrl,
                message = "Connected to WifiCity."
            )
        }

        if (explicitAuthFail) {
            return LoginResult(
                ok = false,
                authFailed = true,
                portalUrl = currentUrl.toString(),
                message = "Invalid campus username or password."
            )
        }

        if (explicitSuccessHint) {
            try {
                Thread.sleep(800)
            } catch (_: InterruptedException) {
            }
            if (probeConnectivity(network, gatewayHints) is ProbeResult.Online) {
                return LoginResult(
                    ok = true,
                    portalUrl = currentUrl.toString(),
                    logoutUrl = logoutUrl,
                    message = "Connected to WifiCity."
                )
            }
        }

        return if (looksLikeLoginForm(postBody)) {
            LoginResult(
                ok = false,
                authFailed = true,
                portalUrl = currentUrl.toString(),
                message = "Portal rejected login. Please check your username and password."
            )
        } else {
            LoginResult(
                ok = false,
                portalUrl = currentUrl.toString(),
                message = "Submitted portal login, waiting for network connectivity."
            )
        }
    }

    fun logout(network: Network, logoutUrl: String) {
        var conn: HttpURLConnection? = null
        try {
            conn = openBoundConnection(network, URL(logoutUrl), timeoutMs = 4000, acceptPortalCerts = true)
            conn.responseCode
        } catch (_: Exception) {
        } finally {
            conn?.disconnect()
        }
    }
}
