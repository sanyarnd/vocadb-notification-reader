package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import vocadb.notification.reader.client.model.enums.Language
import vocadb.notification.reader.client.model.enums.Status
import java.time.LocalDateTime

data class EntryThumbContract constructor(
    val entryType: EntryType? = null,
    val id: Int? = null,
    val mime: String? = null,
    val version: Int? = null
)

data class EntryThumbForApiContract constructor(
    val mime: String? = null,
    val urlSmallThumb: String? = null,
    val urlThumb: String? = null,
    val urlTinyThumb: String? = null
)

enum class EntryType(@JsonValue val value: String) {
    UNDEFINED("Undefined"),
    ALBUM("Album"),
    ARTIST("Artist"),
    DISCUSSION_TOPIC("DiscussionTopic"),
    PV("PV"),
    RELEASE_EVENT("ReleaseEvent"),
    RELEASE_EVENT_SERIES("ReleaseEventSeries"),
    SONG("Song"),
    SONG_LIST("SongList"),
    TAG("Tag"),
    USER("User");
}

data class EntryForApiContract constructor(
    val activityDate: LocalDateTime? = null,
    val additionalNames: String? = null,
    val artistString: String? = null,
    val artistType: ArtistType? = null,
    val createDate: LocalDateTime? = null,
    val defaultName: String? = null,
    val defaultNameLanguage: Language? = null,
    val description: String? = null,
    val discType: DiscType? = null,
    val entryType: EntryType? = null,
    val eventCategory: EventCategory? = null,
    val id: Int? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringContract>? = null,
    val pVs: Collection<PVContract>? = null,
    val songListFeaturedCategory: SongFeaturedCategory? = null,
    val songType: SongType? = null,
    val status: Status? = null,
    val releaseEventSeriesName: String? = null,
    val tagCategoryName: String? = null,
    val tags: Collection<TagUsageForApiContract>? = null,
    val urlSlug: String? = null,
    val version: Int? = null,
    val webLinks: Collection<ArchivedWebLinkContract>? = null
)

enum class EventCategory(@JsonValue val value: String) {
    UNSPECIFIED("Unspecified"),
    ALBUM_RELEASE("AlbumRelease"),
    ANNIVERSARY("Anniversary"),
    CLUB("Club"),
    CONCERT("Concert"),
    CONTEST("Contest"),
    CONVENTION("Convention"),
    OTHER("Other");
}
