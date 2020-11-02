package vocadb.notification.reader.security;

import java.util.Collection;
import lombok.Getter;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.GrantedAuthority;

@Getter
public class CustomUsernamePasswordAuthenticationToken extends UsernamePasswordAuthenticationToken {
    private final LoginService loginService;

    public CustomUsernamePasswordAuthenticationToken(
            Object principal,
            Object credentials,
            Collection<? extends GrantedAuthority> authorities,
            LoginService loginService
    ) {
        super(principal, credentials, authorities);
        this.loginService = loginService;
    }
}
