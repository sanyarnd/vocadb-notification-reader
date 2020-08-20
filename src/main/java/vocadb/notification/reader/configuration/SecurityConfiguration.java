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
import vocadb.notification.reader.service.security.Authority;

@Configuration
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
                .httpBasic().disable()
                // TODO check/enable later
                .requestCache().disable()
                .csrf().disable()
                .cors().disable()
                .headers().disable()

                .formLogin(e -> {
                    e.loginPage("/login");
                    e.requiresAuthenticationMatcher(
                            ServerWebExchangeMatchers.pathMatchers(HttpMethod.POST, "/login/authentication")
                    );
                    e.authenticationFailureHandler(problemAuthenticationHandler);
                    e.authenticationSuccessHandler(this::successHandler);
                })
                .logout(e -> e.logoutSuccessHandler(this::successHandler))
                .anonymous().and()
                .authorizeExchange(e -> {
                    e.pathMatchers(HttpMethod.OPTIONS, "/**").permitAll();
                    e.pathMatchers("/login", "/login/**").permitAll();
                    e.pathMatchers("/webjars/**").permitAll();
                    e.pathMatchers("/swagger-ui*", "/v3/**").permitAll();
                    e.pathMatchers("/**").hasAuthority(Authority.ROLE_USER);
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
