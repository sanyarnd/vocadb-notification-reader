package vocadb.notification.reader.configuration

import mu.KotlinLogging
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Import
import org.springframework.core.ResolvableType
import org.springframework.core.codec.Hints
import org.springframework.http.HttpMethod
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.http.codec.json.Jackson2JsonEncoder
import org.springframework.security.authentication.AccountStatusException
import org.springframework.security.authentication.AuthenticationServiceException
import org.springframework.security.authentication.BadCredentialsException
import org.springframework.security.config.annotation.web.reactive.EnableWebFluxSecurity
import org.springframework.security.config.web.server.ServerHttpSecurity
import org.springframework.security.core.Authentication
import org.springframework.security.core.AuthenticationException
import org.springframework.security.web.server.SecurityWebFilterChain
import org.springframework.security.web.server.WebFilterExchange
import org.springframework.security.web.server.authentication.RedirectServerAuthenticationEntryPoint
import org.springframework.security.web.server.authentication.ServerAuthenticationFailureHandler
import org.springframework.security.web.server.util.matcher.ServerWebExchangeMatchers
import org.springframework.stereotype.Component
import org.springframework.web.server.ServerWebExchange
import org.zalando.problem.Problem
import org.zalando.problem.Status
import org.zalando.problem.spring.webflux.advice.security.SecurityProblemSupport
import reactor.core.publisher.Mono
import vocadb.notification.reader.service.security.UserAuthority
import java.io.IOException

private val logger = KotlinLogging.logger { }

@EnableWebFluxSecurity
@Import(SecurityProblemSupport::class)
class SecurityConfiguration(
    val problemSupport: SecurityProblemSupport,
    val problemAuthenticationHandler: ServerAuthenticationFailureHandler
) {

    @Bean
    fun securityWebFilterChain(http: ServerHttpSecurity): SecurityWebFilterChain {
        return http
            .exceptionHandling {
                it.authenticationEntryPoint { exchange: ServerWebExchange, e: AuthenticationException ->
                    val redirect = RedirectServerAuthenticationEntryPoint("/login")

                    val request = exchange.request
                    val path = request.uri.path
                    when {
                        // redirect if request is made to non-api endpoints
                        !path.startsWith("/api") && !path.startsWith("/login/authentication") ->
                            redirect.commence(exchange, e)
                        else -> problemSupport.commence(exchange, e)
                    }
                }
                it.accessDeniedHandler(problemSupport)
            }
            .httpBasic().disable()
            // TODO check/enable later
            .requestCache().disable()
            .csrf().disable()
            .cors().disable()
            .headers().disable()

            .formLogin {
                it.loginPage("/login")
                it.requiresAuthenticationMatcher(
                    ServerWebExchangeMatchers.pathMatchers(
                        HttpMethod.POST,
                        "/login/authentication"
                    )
                )
                it.authenticationFailureHandler(problemAuthenticationHandler)
                it.authenticationSuccessHandler(this::successHandler)
            }
            .logout {
                it.logoutSuccessHandler(this::successHandler)
            }
            .authorizeExchange {
                it.pathMatchers(HttpMethod.OPTIONS, "/**").permitAll()
                it.pathMatchers("/js/**", "/css/**", "/img/**", "/favicon.ico").permitAll()
                it.pathMatchers("/login", "/login/**").permitAll()

                it.pathMatchers("/**").hasAuthority(UserAuthority.ROLE_USER)
                it.anyExchange().authenticated()
            }
            .build()
    }

    private fun successHandler(webFilterExchange: WebFilterExchange, authentication: Authentication?): Mono<Void> {
        webFilterExchange.exchange.response.statusCode = HttpStatus.OK
        return Mono.empty()
    }
}

@Component
class ProblemAuthenticationFailureHandler(
    val encoder: Jackson2JsonEncoder
) : ServerAuthenticationFailureHandler {

    override fun onAuthenticationFailure(
        webFilterExchange: WebFilterExchange,
        exception: AuthenticationException
    ): Mono<Void> {
        val response = webFilterExchange.exchange.response
        if (response.isCommitted) {
            logger.error {
                "Response is already committed. Unable to write exception " +
                        "${exception.javaClass.simpleName}: ${exception.localizedMessage}"
            }
            return Mono.empty()
        }

        val (status, problem) = exceptionMapper(exception)
        try {
            response.statusCode = status
            return toJson(problem, webFilterExchange.exchange)
        } catch (e: IOException) {
            logger.error(e) { "Failed to write error response" }
        }

        return Mono.empty()
    }

    private fun toJson(body: Any, exchange: ServerWebExchange) =
        exchange.response.writeWith(
            encoder.encode(
                Mono.just(body),
                exchange.response.bufferFactory(),
                ResolvableType.forInstance(body),
                MediaType.APPLICATION_JSON,
                Hints.from(Hints.LOG_PREFIX_HINT, exchange.logPrefix)
            )
        )

    private fun exceptionMapper(exception: AuthenticationException): Pair<HttpStatus, Problem> {
        val problem: Problem = when (exception) {
            is AuthenticationServiceException -> Problem.valueOf(Status.INTERNAL_SERVER_ERROR)
            is AccountStatusException -> Problem.valueOf(
                Status.FORBIDDEN,
                exception.message ?: "User is either disabled or has expired credentials"
            )
            is BadCredentialsException -> Problem.valueOf(
                Status.UNAUTHORIZED,
                "Invalid credentials"
            )
            else -> Problem.valueOf(Status.UNAUTHORIZED)
        }
        val status = when (exception) {
            is AuthenticationServiceException -> Status.INTERNAL_SERVER_ERROR
            is AccountStatusException -> Status.FORBIDDEN
            else -> Status.UNAUTHORIZED
        }

        return Pair(HttpStatus.valueOf(status.statusCode), problem)
    }
}
