package vocadb.notification.reader.client

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.paramnames.ParameterNamesModule
import mu.KotlinLogging
import vocadb.notification.reader.client.api.VocaDbSongApi
import vocadb.notification.reader.client.api.VocaDbUserApi
import java.net.CookieManager
import java.net.ProxySelector
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpRequest.BodyPublishers
import java.net.http.HttpResponse.BodyHandlers
import java.time.Duration
import java.util.concurrent.CompletableFuture

private val logger = KotlinLogging.logger { }

private const val SET_COOKIE: String = "Set-Cookie"

class VocaDbClient(
    /** Object mapper for serialization/deserialization */
    objectMapper: ObjectMapper =
        jacksonObjectMapper().registerModule(JavaTimeModule())
            .registerModule(Jdk8Module())
            .registerModule(ParameterNamesModule())
            .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS),
    /** Requests' thread pool */

    /** Service's base URL */
    val baseUrl: URI = URI("https://vocadb.net"),

    // misc parameters, probably won't need it at all
    proxySelector: ProxySelector = HttpClient.Builder.NO_PROXY,
    connectTimeout: Duration = Duration.ofSeconds(30),
    followRedirects: HttpClient.Redirect = HttpClient.Redirect.NEVER,

    private val cookieManager: CookieManager = CookieManager()
) {
    private val client = HttpClient
        .newBuilder()
        .connectTimeout(connectTimeout)
        .followRedirects(followRedirects)
        .cookieHandler(cookieManager)
        .proxy(proxySelector)
        .version(HttpClient.Version.HTTP_1_1)
        .build()

    private var _cookies: List<String> = emptyList()
    var cookies: List<String>
        get() = _cookies
        set(value) {
            _cookies = value
            cookieManager.put(baseUrl, mapOf(SET_COOKIE to _cookies))
        }

    val userApi: VocaDbUserApi = VocaDbUserApi(client, baseUrl, objectMapper)
    val songApi: VocaDbSongApi = VocaDbSongApi(client, baseUrl, objectMapper)

    fun authenticate(user: String, password: String): CompletableFuture<Boolean> {
        logger.info { "Authenticating user '$user'" }

        val request: HttpRequest = HttpRequest.newBuilder()
            .uri(baseUrl.resolve("/User/Login?UserName=$user&Password=$password"))
            .POST(BodyPublishers.noBody())
            .build()

        return client.sendAsync(request, BodyHandlers.discarding())
            .thenApply {
                val authenticated = isAuthenticated()
                // intercept request and fetch original cookies
                // we need that to get original cookies
                cookies = it.headers().allValues(SET_COOKIE)
                logger.info { "User '$user' is authenticated = $authenticated" }
                authenticated
            }
    }

    fun isAuthenticated(): Boolean {
        val cookies = cookieManager.cookieStore[baseUrl]
        return cookies.isNotEmpty() && cookies.all { !it.hasExpired() }
    }

}
