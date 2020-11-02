package vocadb.notification.reader.web;

import java.util.List;
import javax.validation.constraints.Max;
import javax.validation.constraints.NotEmpty;
import javax.validation.constraints.PositiveOrZero;
import lombok.RequiredArgsConstructor;
import org.springframework.http.MediaType;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.validation.annotation.Validated;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.client.query.LanguagePreference;
import vocadb.notification.reader.security.SecurityPrincipal;
import vocadb.notification.reader.service.NotificationService;
import vocadb.notification.reader.web.dto.NotificationsDto;

@RestController
@Validated
@RequestMapping(value = "/api/v1/notifications", produces = MediaType.APPLICATION_JSON_VALUE)
@RequiredArgsConstructor
public class NotificationController {

    private final NotificationService notificationService;

    @GetMapping
    public Mono<NotificationsDto> getNotifications(
            @AuthenticationPrincipal SecurityPrincipal securityPrincipal,
            @RequestParam @PositiveOrZero int startOffset,
            @RequestParam @PositiveOrZero @Max(100) int maxResults,
            @RequestParam LanguagePreference languagePreference
    ) {
        int userId = securityPrincipal.vocadbUser().id();
        List<String> authCookies = securityPrincipal.cookies();

        return Mono.fromCompletionStage(
                notificationService.loadNotifications(userId, startOffset, maxResults, authCookies, languagePreference))
                .map(p -> new NotificationsDto(p.getLeft(), p.getRight()));
    }

    @DeleteMapping
    public Mono<Void> deleteNotifications(
            @AuthenticationPrincipal SecurityPrincipal securityPrincipal,
            @RequestParam @NotEmpty List<Integer> notificationIds
    ) {
        int userId = securityPrincipal.vocadbUser().id();
        List<String> authCookies = securityPrincipal.cookies();
        return notificationService.deleteNotifications(userId, notificationIds, authCookies);
    }
}


