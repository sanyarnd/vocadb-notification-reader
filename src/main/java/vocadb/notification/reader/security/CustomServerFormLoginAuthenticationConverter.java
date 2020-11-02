package vocadb.notification.reader.security;

import org.springframework.security.authentication.AuthenticationServiceException;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.web.server.authentication.ServerFormLoginAuthenticationConverter;
import org.springframework.web.server.ServerWebExchange;
import reactor.core.publisher.Mono;

/**
 * Custom form handler, supports additional data
 */
public class CustomServerFormLoginAuthenticationConverter extends ServerFormLoginAuthenticationConverter {
    public static final String LOGIN_SERVICE_FIELD = "login-service";

    @Override
    @SuppressWarnings("ReturnCount")
    public Mono<Authentication> convert(ServerWebExchange exchange) {
        Mono<UsernamePasswordAuthenticationToken> token = super.convert(exchange)
            .map(a -> (UsernamePasswordAuthenticationToken) a);

        Mono<LoginService> loginService = exchange.getFormData()
            .flatMap(formData -> {
                String loginServiceField = formData.getFirst(LOGIN_SERVICE_FIELD);
                if (loginServiceField == null) {
                    return Mono.empty();
                }
                try {
                    return Mono.just(LoginService.valueOf(loginServiceField));
                } catch (IllegalArgumentException ex) {
                    return Mono.empty();
                }
            })
            .switchIfEmpty(Mono.error(() ->
                new AuthenticationServiceException("Missing '" + LOGIN_SERVICE_FIELD + "' field")));

        return loginService.zipWith(token)
            .map(t -> {
                    UsernamePasswordAuthenticationToken authenticationToken = t.getT2();
                    LoginService authority = t.getT1();
                    return new CustomUsernamePasswordAuthenticationToken(
                        authenticationToken.getPrincipal(),
                        authenticationToken.getCredentials(),
                        authenticationToken.getAuthorities(),
                        authority
                    );
                }
            );
    }
}
