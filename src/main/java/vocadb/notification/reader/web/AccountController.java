package vocadb.notification.reader.web;

import org.springframework.http.MediaType;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.security.VocaDbPrincipal;
import vocadb.notification.reader.web.dto.AccountDto;

@RestController
@RequestMapping(value = "api/v1/account", produces = MediaType.APPLICATION_JSON_VALUE)
public class AccountController {
    @GetMapping
    public Mono<AccountDto> currentUser(@AuthenticationPrincipal Mono<VocaDbPrincipal> principal) {
        return principal.map(p -> new AccountDto(p.vocadbUser().id(), p.vocadbUser().name()));
    }
}

