package vocadb.notification.reader.security;

import lombok.RequiredArgsConstructor;
import org.springframework.security.core.GrantedAuthority;

@RequiredArgsConstructor
public enum VocadbRole implements GrantedAuthority {
    VOCADB(Name.VOCADB),
    TOUHOU(Name.TOUHOUDB),
    TAITE(Name.TAITEDB);

    private final String authority;

    public String getAuthority() {
        return authority;
    }

    public interface Name {
        String VOCADB = "ROLE_VOCADB";
        String TOUHOUDB = "ROLE_TOUHOUDB";
        String TAITEDB = "ROLE_TAITEDB";
    }
}
