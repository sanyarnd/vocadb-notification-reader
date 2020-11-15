package vocadb.notification.reader.web;

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.stream.Collectors;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.BodyInserters;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.client.query.LanguagePreference;
import vocadb.notification.reader.security.SecurityPrincipal;
import vocadb.notification.reader.service.NotificationService;
import vocadb.notification.reader.service.dto.AccountDto;

@Component
@RequiredArgsConstructor
public class WebHandlers {
    private final NotificationService notificationService;

    public @NotNull Mono<ServerResponse> getAccount(ServerRequest req) {
        return req.principal()
            .map(p -> (SecurityPrincipal) ((UsernamePasswordAuthenticationToken) p).getPrincipal())
            .map(p -> new AccountDto(p.vocadbUser().id(), p.vocadbUser().name()))
            .flatMap(dto -> ServerResponse.ok().bodyValue(dto));
    }

    @SuppressWarnings("MagicNumber")
    public @NotNull Mono<ServerResponse> getNotifications(ServerRequest req) {
        return req.principal()
            .map(p -> (SecurityPrincipal) ((UsernamePasswordAuthenticationToken) p).getPrincipal())
            .flatMap(p -> {
                int userId = p.vocadbUser().id();
                List<String> authCookies = p.cookies();

                var startOffset = req.queryParam("startOffset")
                    .map(Integer::parseInt)
                    .filter(o -> o >= 0)
                    .orElseThrow();
                var maxResults = req.queryParam("maxResults")
                    .map(Integer::parseInt)
                    .filter(r -> r >= 0 && r <= 100)
                    .orElseThrow();
                LanguagePreference languagePreference = req.queryParam("languagePreference")
                    .map(LanguagePreference::valueOf)
                    .orElseThrow();

                return notificationService
                    .loadNotifications(userId, startOffset, maxResults, authCookies, languagePreference);
            })
            .flatMap(dto -> ServerResponse.ok().bodyValue(dto));
    }

    public @NotNull Mono<ServerResponse> deleteNotifications(ServerRequest req) {
        return req.principal()
            .map(p -> (SecurityPrincipal) ((UsernamePasswordAuthenticationToken) p).getPrincipal())
            .flatMap(p -> {
                int userId = p.vocadbUser().id();
                List<String> authCookies = p.cookies();

                Collection<Integer> notificationIds = req.queryParam("notificationIds")
                    .map(n ->
                        Arrays.stream(n.split(","))
                            .map(Integer::parseInt)
                            .collect(Collectors.toList())
                    )
                    .orElseThrow();

                return notificationService.deleteNotifications(userId, notificationIds, authCookies);
            })
            .flatMap(dto -> ServerResponse.ok().bodyValue(BodyInserters.empty()));
    }
}
