package vocadb.notification.reader.configuration;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import java.nio.charset.StandardCharsets;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.buffer.DataBuffer;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.server.reactive.ServerHttpResponse;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.config.web.server.ServerHttpSecurity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.AuthenticationException;
import org.springframework.security.web.server.SecurityWebFilterChain;
import org.springframework.security.web.server.WebFilterExchange;
import org.springframework.security.web.server.authentication.AuthenticationWebFilter;
import org.springframework.security.web.server.util.matcher.ServerWebExchangeMatchers;
import org.springframework.web.server.ServerWebExchange;
import org.springframework.web.server.WebFilter;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.security.CustomServerFormLoginAuthenticationConverter;
import static org.springframework.http.HttpStatus.FORBIDDEN;
import static org.springframework.http.HttpStatus.UNAUTHORIZED;

@Configuration(proxyBeanMethods = false)
@RequiredArgsConstructor
public class SecurityConfiguration {

    @Bean
    @SuppressWarnings("MultipleStringLiterals")
    public SecurityWebFilterChain securityWebFilterChain(final ServerHttpSecurity http) {
        SecurityWebFilterChain build = http.exceptionHandling(it -> {
            it.authenticationEntryPoint(this::authEntryPoint);
            it.accessDeniedHandler(this::accessDeniedHandler);
        })
            // TODO check/enable later
            .httpBasic().disable()
            .requestCache().disable()
            .csrf().disable()
            .cors().disable()
            .headers().disable()
            .anonymous().disable()

            .formLogin(e -> {
                e.loginPage("/api/login");
                e.requiresAuthenticationMatcher(
                    ServerWebExchangeMatchers.pathMatchers(HttpMethod.POST, "/api/login/authentication")
                );
                e.authenticationFailureHandler(this::failureHandler);
                e.authenticationSuccessHandler(this::successHandler);
            })
            .logout(e -> {
                e.logoutSuccessHandler(this::successHandler);
                e.logoutUrl("/api/logout");
            })

            .authorizeExchange(e -> {
                // permit all preflight requests
                e.pathMatchers(HttpMethod.OPTIONS).permitAll();

                // allow an arbitrary access to login and swagger endpoints
                e.pathMatchers("/api/login/**", "/api/logout").permitAll();

                // everything else must be authenticated
                e.anyExchange().authenticated();
            })
            .build();

        // currently formLogin builder does not allow to customize authentication converter
        build.getWebFilters().collectList().subscribe(
            webFilters -> {
                for (WebFilter filter : webFilters) {
                    if (filter instanceof AuthenticationWebFilter) {
                        AuthenticationWebFilter awf = (AuthenticationWebFilter) filter;
                        awf.setServerAuthenticationConverter(new CustomServerFormLoginAuthenticationConverter());
                    }
                }
            }
        );

        return build;
    }

    @SuppressFBWarnings("UP_UNUSED_PARAMETER")
    private Mono<Void> authEntryPoint(ServerWebExchange serverWebExchange, AuthenticationException exception) {
        return authErrorHandler(serverWebExchange, UNAUTHORIZED);
    }

    private String getErrorBody(HttpStatus unauthorized) {
        return String.format(
            "{\"status\": \"%d\", \"title\":\"%s\"}",
            unauthorized.value(), unauthorized.getReasonPhrase()
        );
    }

    @SuppressFBWarnings("UP_UNUSED_PARAMETER")
    private Mono<Void> accessDeniedHandler(ServerWebExchange serverWebExchange, AccessDeniedException e) {
        return authErrorHandler(serverWebExchange, FORBIDDEN);
    }

    @NotNull private Mono<Void> authErrorHandler(ServerWebExchange serverWebExchange, HttpStatus forbidden) {
        ServerHttpResponse response = serverWebExchange.getResponse();
        final String body = getErrorBody(forbidden);

        response.setStatusCode(forbidden);
        response.getHeaders().setContentType(MediaType.parseMediaType("application/problem+json"));

        DataBuffer buf = response.bufferFactory().wrap(body.getBytes(StandardCharsets.UTF_8));
        return response.writeWith(Mono.just(buf));
    }

    @SuppressFBWarnings("UP_UNUSED_PARAMETER")
    private Mono<Void> failureHandler(WebFilterExchange webFilterExchange, AuthenticationException exception) {
        return authErrorHandler(webFilterExchange.getExchange(), UNAUTHORIZED);
    }

    @SuppressFBWarnings("UP_UNUSED_PARAMETER")
    private Mono<Void> successHandler(WebFilterExchange webFilterExchange, Authentication authentication) {
        webFilterExchange.getExchange().getResponse().setStatusCode(HttpStatus.OK);
        return Mono.empty();
    }
}
