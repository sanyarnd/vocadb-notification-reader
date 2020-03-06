package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import java.time.LocalDateTime

data class UserForApiContract constructor(
    val id: Long,
    val name: String,
    val active: Boolean? = null,
    val groupId: GroupId? = null,
    val knownLanguages: Collection<UserKnownLanguageContract>? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val memberSince: LocalDateTime? = null,
    val oldUsernames: Collection<OldUsernameContract>? = null,
    val verifiedArtist: Boolean? = null
)

data class UserKnownLanguageContract constructor(
    val cultureCode: String? = null,
    val proficiency: Proficiency? = null
)

data class UserMessageContract constructor(
    val id: Long,
    val subject: String,
    val body: String? = null,
    val createdFormatted: String? = null,
    val highPriority: Boolean? = null,
    val inbox: Inbox? = null,
    val read: Boolean? = null,
    val receiver: UserForApiContract? = null,
    val sender: UserForApiContract? = null
)

enum class GroupId(@JsonValue val value: String) {
    NOTHING("Nothing"),
    LIMITED("Limited"),
    REGULAR("Regular"),
    TRUSTED("Trusted"),
    MODERATOR("Moderator"),
    ADMIN("Admin");
}

enum class Inbox(@JsonValue val value: String) {
    NOTHING("Nothing"),
    RECEIVED("Received"),
    SENT("Sent"),
    NOTIFICATIONS("Notifications");
}

enum class Proficiency(@JsonValue val value: String) {
    NOTHING("Nothing"),
    BASICS("Basics"),
    INTERMEDIATE("Intermediate"),
    ADVANCED("Advanced"),
    NATIVE("Native");
}

data class UserWithEmailContract constructor(
    val email: String? = null,
    val id: Int? = null,
    val name: String? = null
)
