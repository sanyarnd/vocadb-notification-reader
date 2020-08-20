package vocadb.notification.reader.service.security;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import java.util.Collection;
import java.util.List;
import lombok.Getter;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.userdetails.User;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@SuppressFBWarnings("EQ_DOESNT_OVERRIDE_EQUALS")
public class VocaDbPrincipal extends User {
    private final UserForApiContract vocadbUser;
    private final List<String> cookies;

    public VocaDbPrincipal(
            UserForApiContract vocadbUser,
            List<String> cookies,
            Collection<? extends GrantedAuthority> authorities
    ) {
        super(
                vocadbUser.name(),
                "not-stored",
                true,
                true,
                true,
                true,
                authorities
        );

        this.vocadbUser = vocadbUser;
        this.cookies = cookies;
    }
}
