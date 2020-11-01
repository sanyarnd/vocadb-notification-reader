package vocadb.notification.reader.configuration;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Import;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.security.config.web.server.ServerHttpSecurity;
import org.springframework.security.core.Authentication;
import org.springframework.security.web.server.SecurityWebFilterChain;
import org.springframework.security.web.server.WebFilterExchange;
import org.springframework.security.web.server.authentication.ServerAuthenticationFailureHandler;
import org.springframework.security.web.server.util.matcher.ServerWebExchangeMatchers;
import org.zalando.problem.spring.webflux.advice.security.SecurityProblemSupport;
import reactor.core.publisher.Mono;

@Configuration(proxyBeanMethods = false)
@Import(SecurityProblemSupport.class)
@RequiredArgsConstructor
public class SecurityConfiguration {
    private final SecurityProblemSupport problemSupport;
    private final ServerAuthenticationFailureHandler problemAuthenticationHandler;

    @Bean
    @SuppressWarnings("MultipleStringLiterals")
    public SecurityWebFilterChain securityWebFilterChain(final ServerHttpSecurity http) {
        return http.exceptionHandling(it -> {
            it.authenticationEntryPoint(problemSupport);
            it.accessDeniedHandler(problemSupport);
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
                    e.authenticationFailureHandler(problemAuthenticationHandler);
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
                    e.pathMatchers("/webjars/**").permitAll();
                    e.pathMatchers("/swagger-ui*", "/v3/**").permitAll();

                    // everything else must be authenticated
                    e.anyExchange().authenticated();
                })
                .build();
    }

    @SuppressFBWarnings("UP_UNUSED_PARAMETER")
    private Mono<Void> successHandler(WebFilterExchange webFilterExchange, Authentication authentication) {
        webFilterExchange.getExchange().getResponse().setStatusCode(HttpStatus.OK);
        return Mono.empty();
    }
}
