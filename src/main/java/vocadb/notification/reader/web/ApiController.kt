package vocadb.notification.reader.web

import org.springframework.security.core.annotation.AuthenticationPrincipal
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RequestParam
import org.springframework.web.bind.annotation.RestController
import vocadb.notification.reader.client.api.enums.LanguagePreference
import vocadb.notification.reader.service.Notification
import vocadb.notification.reader.service.NotificationService
import vocadb.notification.reader.service.security.VocaDbPrincipal

@RestController
class ApiController(
    val notificationService: NotificationService
) {
    @GetMapping("/api/account/data")
    suspend fun accountData(@AuthenticationPrincipal principal: VocaDbPrincipal): UserDataDTO {
        return UserDataDTO(principal.vocadbUser.id, principal.vocadbUser.name)
    }

    @GetMapping("/api/account/notifications")
    suspend fun getNotifications(
        @AuthenticationPrincipal principal: VocaDbPrincipal,
        @RequestParam startOffset: Long,
        @RequestParam maxResults: Long,
        @RequestParam languagePreference: LanguagePreference
    ): NotificationsDTO {
        require(startOffset >= 0) { "startOffset must be >= 0" }
        require(maxResults in 1..100) { "maxResults must be in range of [1, 100]" }

        val userId = principal.vocadbUser.id
        val authCookies = principal.cookies

        val notifications = notificationService
            .loadNotifications(userId, startOffset, maxResults, authCookies, languagePreference)

        return NotificationsDTO(notifications.first, notifications.second)
    }

    @DeleteMapping("/api/account/notifications")
    suspend fun deleteNotifications(
        @AuthenticationPrincipal principal: VocaDbPrincipal,
        @RequestParam notificationIds: Collection<Long>
    ): Unit {
        require(notificationIds.isNotEmpty())

        val userId = principal.vocadbUser.id
        val authCookies = principal.cookies

        notificationService.deleteNotifications(userId, notificationIds, authCookies)
    }
}

data class NotificationsDTO(
    val notifications: Collection<Notification>,
    val totalCount: Long
)

class UserDataDTO(
    val vocaDbId: Long,
    val username: String
)
