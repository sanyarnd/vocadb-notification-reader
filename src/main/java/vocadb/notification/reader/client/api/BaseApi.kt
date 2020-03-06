package vocadb.notification.reader.client.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.convertValue
import com.fasterxml.jackson.module.kotlin.jacksonTypeRef
import java.io.InputStream
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse
import java.net.http.HttpResponse.BodyHandlers
import java.util.concurrent.CompletableFuture

abstract class BaseApi(
    protected val client: HttpClient,
    protected val baseUrl: URI,
    protected val mapper: ObjectMapper
) {
    protected fun URI.getRequest(): HttpRequest = HttpRequest.newBuilder()
        .header("Accept", "application/json").uri(this).GET().build()

    protected fun URI.deleteRequest(): HttpRequest = HttpRequest.newBuilder().uri(this).DELETE().build()

    protected inline fun <reified T> sendAsync(
        request: HttpRequest,
        typeReference: TypeReference<T> = jacksonTypeRef()
    ): CompletableFuture<T> {

        return client.sendAsync(request, BodyHandlers.ofInputStream())
            .thenApply(HttpResponse<InputStream>::body)
            .thenApply {
                mapper.readValue(it, typeReference)
            }
    }

    protected fun sendAsyncVoid(request: HttpRequest): CompletableFuture<Void> {
        return client.sendAsync(request, BodyHandlers.discarding()).thenApply { it.body() }
    }

    /** Convert params map to URL query */
    protected fun Map<*, *>.toQuery(): String = this.asSequence()
        // null values are skipped
        .filter { (_, v) -> v != null }
        .map { (k, v) ->
            val value: String = if (v is Collection<*>) v.toQueryString()
            else mapper.convertValue(v!!)

            Pair(k, value)
        }
        .joinToString("&") { (k, v) -> "$k=$v" }

    /**
     * Special collection stringifier.
     * There is no collections in API with depth more than 1 (collection of collections),
     * no point to handle this recursive case
     */
    private fun Collection<*>.toQueryString(): String = this
        .map { mapper.convertValue<String>(it!!) }
        .joinToString(",") { it }
}
