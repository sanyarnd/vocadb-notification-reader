package vocadb.notification.reader.service

import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.future.asDeferred
import kotlinx.coroutines.future.await
import org.springframework.beans.factory.ObjectProvider
import org.springframework.stereotype.Service
import vocadb.notification.reader.client.VocaDbClient
import vocadb.notification.reader.client.api.enums.LanguagePreference
import vocadb.notification.reader.client.api.enums.OptionalFields
import vocadb.notification.reader.client.model.Inbox
import vocadb.notification.reader.client.model.PVContract
import vocadb.notification.reader.client.model.PvService
import vocadb.notification.reader.client.model.PvType
import vocadb.notification.reader.client.model.SongType
import vocadb.notification.reader.client.model.TagUsageForApiContract
import vocadb.notification.reader.client.model.UserMessageContract
import java.time.LocalDateTime

private typealias PayloadId = Long
private typealias TotalCount = Long

@Service
class NotificationService(val clientProvider: ObjectProvider<VocaDbClient>) {

    suspend fun deleteNotifications(userId: Long, notificationIds: Collection<Long>, authCookies: List<String>) {
        val client = clientProvider.getObject()
        client.cookies = authCookies

        client.userApi.deleteMessages(userId, notificationIds)
    }

    suspend fun loadNotifications(
        userId: Long,
        startOffset: Long,
        maxResults: Long,
        authCookies: List<String>,
        languagePreference: LanguagePreference
    ): Pair<Collection<Notification>, TotalCount> {
        val rawNotifications = loadRawNotifications(userId, startOffset, maxResults, authCookies)
        val notifications = loadNotificationsData(rawNotifications.notifications, authCookies, languagePreference)

        return Pair(notifications, rawNotifications.totalCount)
    }

    private suspend fun loadRawNotifications(
        userId: Long,
        startOffset: Long,
        maxResults: Long,
        authCookies: List<String>
    ): RawNotificationsResult {
        val client = clientProvider.getObject()
        client.cookies = authCookies

        val messages = client.userApi.getMessages(
            inbox = Inbox.NOTIFICATIONS,
            getTotalCount = true,
            id = userId,
            start = startOffset,
            maxResults = maxResults
        ).await()

        if (messages.items.isNullOrEmpty()) return RawNotificationsResult(emptyList(), 0)

        val notifications = messages.items.map { client.userApi.getMessage(it.id).asDeferred() }.awaitAll()
        return RawNotificationsResult(notifications, messages.totalCount!!)
    }

    private suspend fun loadNotificationsData(
        notifications: Collection<UserMessageContract>,
        authCookies: List<String>,
        languagePreference: LanguagePreference
    ): Collection<Notification> {
        if (notifications.isEmpty()) return emptyList()

        val client = clientProvider.getObject()
        client.cookies = authCookies

        return notifications
            .map { n ->
                val (type, matchedId) = detectNotificationType(n)
                val originalBody = n.body ?: "<empty body>"

                when (type) {
                    NotificationType.SONG -> createSongNotification(
                        client,
                        n.id,
                        n.subject,
                        originalBody,
                        matchedId,
                        languagePreference
                    )
                    NotificationType.ARTIST -> ArtistNotification(n.id, n.subject, originalBody)
                    NotificationType.EVENT -> EventNotification(n.id, n.subject, originalBody)
                    NotificationType.ALBUM -> AlbumNotification(n.id, n.subject, originalBody)
                    NotificationType.REPORT -> ReportNotification(n.id, n.subject, originalBody)
                    NotificationType.UNKNOWN -> UnknownNotification(n.id, n.subject, originalBody)
                }
            }
    }

    private suspend fun createSongNotification(
        client: VocaDbClient,
        id: Long,
        subject: String,
        body: String,
        songId: Long,
        languagePreference: LanguagePreference
    ): SongNotification {
        val song = client.songApi
            .getSongById(songId, listOf(OptionalFields.TAGS, OptionalFields.PVS), languagePreference)
            .await()

        val type = when {
            subject.contains("tagged") -> SongNotificationType.TAGGED
            else -> SongNotificationType.NEW
        }

        val artist = song.artistString
        val title = song?.name
        val publishDate = song?.publishDate
        val songType = song.songType
        val tags = (song.tags ?: emptyList())
            .map(Tag.Companion::of)
            .sortedWith(compareBy<Tag> { it.count }
                .thenBy { it.name }
            )
        val pvs = (song.pvs ?: emptyList())
            .map(PV.Companion::of)
            .sortedWith(compareBy<PV> { it.service.value }
                .thenBy { it.pvType }
            )

        return SongNotification(id, subject, body, type, songId, songType, title, artist, tags, pvs, publishDate)
    }

    private fun detectNotificationType(notification: UserMessageContract): Pair<NotificationType, PayloadId> {
        val unknown = Pair(NotificationType.UNKNOWN, 0L)

        val pattern = Regex(""".*\(https?://vocadb.net/(\w+)/(\d+)\).*""")
        val match = pattern.find(notification.body!!) ?: return unknown

        val type = NotificationType.fromCode(match.groupValues[1])
        if (type == NotificationType.UNKNOWN) return unknown

        val matchedId = match.groupValues[2].toLong()
        return Pair(type, matchedId)
    }
}

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "notificationType")
@JsonSubTypes(
    JsonSubTypes.Type(value = AlbumNotification::class, name = "AlbumNotification"),
    JsonSubTypes.Type(value = ArtistNotification::class, name = "ArtistNotification"),
    JsonSubTypes.Type(value = EventNotification::class, name = "EventNotification"),
    JsonSubTypes.Type(value = ReportNotification::class, name = "ReportNotification"),
    JsonSubTypes.Type(value = SongNotification::class, name = "SongNotification"),
    JsonSubTypes.Type(value = UnknownNotification::class, name = "UnknownNotification")
)
sealed class Notification(
    val id: Long,
    val originalSubject: String,
    val originalBody: String
)

class AlbumNotification(
    id: Long,
    originalSubject: String,
    originalBody: String
) : Notification(id, originalSubject, originalBody)

class ArtistNotification(
    id: Long,
    originalSubject: String,
    originalBody: String
) : Notification(id, originalSubject, originalBody)

class EventNotification(
    id: Long,
    originalSubject: String,
    originalBody: String
) : Notification(id, originalSubject, originalBody)

class ReportNotification(
    id: Long,
    originalSubject: String,
    originalBody: String
) : Notification(id, originalSubject, originalBody)

class SongNotification(
    id: Long,
    originalSubject: String,
    originalBody: String,
    val type: SongNotificationType,
    val songId: Long,
    val songType: SongType,
    val title: String? = null,
    val artist: String? = null,
    val tags: Collection<Tag>,
    val pvs: Collection<PV>,
    val releaseDate: LocalDateTime? = null
) : Notification(id, originalSubject, originalBody)

enum class SongNotificationType {
    TAGGED,
    NEW
}

data class PV(
    val id: Long,
    val pvType: PvType,
    val service: PvService,
    val url: String,
    val name: String,
    val disabled: Boolean,
    val author: String? = null,
    val publishDate: LocalDateTime? = null,
    val pvId: String? = null,
    val thumbUrl: String? = null,
    val timestamp: String? = null
) {
    companion object {
        fun of(pv: PVContract): PV {
            val timestamp = if (pv.service == PvService.PIAPRO && pv.extendedMetadata?.json != null) {
                pv.extendedMetadata.json["Timestamp"]
            } else {
                null
            }

            return PV(
                pv.id,
                pv.pvType,
                pv.service,
                pv.url,
                pv.name,
                pv.disabled,
                pv.author,
                pv.publishDate,
                pv.pvId,
                pv.thumbUrl,
                timestamp
            )
        }
    }
}

data class Tag(
    val count: Long,
    val id: Long,
    val name: String,
    val categoryName: String? = null
) {
    companion object {
        fun of(tag: TagUsageForApiContract): Tag {
            return Tag(tag.count, tag.tag.id, tag.tag.name, tag.tag.categoryName)
        }
    }
}

class UnknownNotification(
    id: Long,
    originalSubject: String,
    originalBody: String
) : Notification(id, originalSubject, originalBody)

data class RawNotificationsResult(
    val notifications: Collection<UserMessageContract>,
    val totalCount: Long
)

enum class NotificationType(val code: String) {
    SONG("S"),
    ARTIST("Ar"),
    EVENT("E"),
    ALBUM("Al"),
    REPORT("R"),
    UNKNOWN("~");

    companion object {
        @JvmStatic
        fun fromCode(code: String): NotificationType = when (code) {
            SONG.code -> SONG
            ARTIST.code -> ARTIST
            EVENT.code -> EVENT
            ALBUM.code -> ALBUM
            REPORT.code -> REPORT
            else -> UNKNOWN
        }

    }
}
