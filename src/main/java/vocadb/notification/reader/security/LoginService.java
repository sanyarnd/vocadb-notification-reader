package vocadb.notification.reader.security;

import lombok.RequiredArgsConstructor;
import org.springframework.security.core.GrantedAuthority;

@RequiredArgsConstructor
public enum LoginService implements GrantedAuthority {
    VOCADB(Name.VOCADB),
    TOUHOU(Name.TOUHOUDB),
    UTAITE(Name.UTAITEDB);

    private final String authority;

    public String getAuthority() {
        return authority;
    }

    public interface Name {
        String VOCADB = "VOCADB";
        String TOUHOUDB = "TOUHOUDB";
        String UTAITEDB = "UTAITEDB";
    }
}
