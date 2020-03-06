package vocadb.notification.reader.web

import org.springframework.stereotype.Component
import org.springframework.web.server.ServerWebExchange
import org.springframework.web.server.WebFilter
import org.springframework.web.server.WebFilterChain
import reactor.core.publisher.Mono

/**
 * For some reasons WebFlux doesn't support forward/redirect
 * commands, so as a workaround there is a custom filter,
 * which returns dedicated .html files
 */
@Component
class RedirectToHtmlFilter : WebFilter {
    override fun filter(exchange: ServerWebExchange, chain: WebFilterChain): Mono<Void> {
        val request = exchange.request
        val path = request.uri.path
        return when {
            path == "/" || path.startsWith("/home") -> chain.filter(
                exchange.mutate().request(
                    request.mutate().path("/home.html").build()
                ).build()
            )
            path.startsWith("/login") -> chain.filter(
                exchange.mutate().request(
                    request.mutate().path("/login.html").build()
                ).build()
            )
            else -> chain.filter(exchange)
        }
    }
}
