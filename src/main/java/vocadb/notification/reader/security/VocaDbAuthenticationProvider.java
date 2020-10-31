package vocadb.notification.reader.security;

import java.util.Collections;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.ObjectProvider;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.ReactiveAuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.client.VocaDbClient;

@Component
@RequiredArgsConstructor
public class VocaDbAuthenticationProvider implements ReactiveAuthenticationManager {
    private final ObjectProvider<VocaDbClient> clientProvider;

    @Override
    public Mono<Authentication> authenticate(Authentication authentication) {
        String username = authentication.getName();
        String credentials = (String) authentication.getCredentials();

        VocaDbClient client = clientProvider.getObject();
        return Mono.fromCompletionStage(client.authenticate(username, credentials))
                .flatMap(isAuthenticated -> isAuthenticated
                        ? Mono.fromCompletionStage(client.userApi().getCurrentUser(Collections.emptyList()))
                        : Mono.error(() -> new BadCredentialsException("Bad credentials")))
                .map(u -> {
                    List<GrantedAuthority> userAuthorities = List.of(VocadbRole.VOCADB);
                    VocaDbPrincipal p = new VocaDbPrincipal(u, List.copyOf(client.cookies()), userAuthorities);
                    return new UsernamePasswordAuthenticationToken(p, p.getPassword(), p.getAuthorities());
                });
    }
}

