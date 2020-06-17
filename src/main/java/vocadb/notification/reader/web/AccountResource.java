package vocadb.notification.reader.web;

import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.service.security.VocaDbPrincipal;
import vocadb.notification.reader.web.dto.AccountDto;

@RestController
@RequestMapping("api/v1/account")
public class AccountResource {
    @GetMapping
    public Mono<AccountDto> currentUser(@AuthenticationPrincipal Mono<VocaDbPrincipal> principal) {
        return principal.map(p -> new AccountDto(p.vocadbUser().id(), p.vocadbUser().name()));
    }
}

