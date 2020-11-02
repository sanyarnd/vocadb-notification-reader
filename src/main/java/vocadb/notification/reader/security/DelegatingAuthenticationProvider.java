package vocadb.notification.reader.security;

import java.util.Collections;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.ObjectProvider;
import org.springframework.security.authentication.AuthenticationServiceException;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.ReactiveAuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.client.BaseClient;
import vocadb.notification.reader.client.TouhouDbClient;
import vocadb.notification.reader.client.UtaiteDbClient;
import vocadb.notification.reader.client.VocaDbClient;

@Component
@RequiredArgsConstructor
public class DelegatingAuthenticationProvider implements ReactiveAuthenticationManager {
    private final ObjectProvider<VocaDbClient> vocaDbClientProvider;
    private final ObjectProvider<UtaiteDbClient> utaiteDbClientProvider;
    private final ObjectProvider<TouhouDbClient> touhouDbClientProvider;

    @Override
    public Mono<Authentication> authenticate(Authentication authentication) {
        if (!(authentication instanceof CustomUsernamePasswordAuthenticationToken)) {
            return Mono.error(() ->
                new AuthenticationServiceException("Invalid token type: " + authentication.getClass().getName()));
        }
        CustomUsernamePasswordAuthenticationToken auth = (CustomUsernamePasswordAuthenticationToken) authentication;

        String username = auth.getName();
        String credentials = (String) auth.getCredentials();
        LoginService loginService = auth.loginService();

        final BaseClient client = getClient(loginService);
        return Mono.fromCompletionStage(client.authenticate(username, credentials))
            .flatMap(isAuthenticated -> isAuthenticated
                ? Mono.fromCompletionStage(client.userApi().getCurrentUser(Collections.emptyList()))
                : Mono.error(() -> new BadCredentialsException("Bad credentials")))
            .map(u -> {
                List<GrantedAuthority> userAuthorities = List.of(loginService);
                SecurityPrincipal p = new SecurityPrincipal(u, List.copyOf(client.cookies()), userAuthorities);
                return new UsernamePasswordAuthenticationToken(p, p.getPassword(), p.getAuthorities());
            });
    }

    private BaseClient getClient(LoginService loginService) {
        final BaseClient client;
        switch (loginService) {
            case VOCADB:
                client = vocaDbClientProvider.getObject();
                break;
            case TOUHOU:
                client = touhouDbClientProvider.getObject();
                break;
            case UTAITE:
                client = utaiteDbClientProvider.getObject();
                break;
            default:
                throw new IllegalStateException("Unexpected value: " + loginService);
        }
        return client;
    }
}

