package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import vocadb.notification.reader.client.model.enums.Language
import vocadb.notification.reader.client.model.enums.Status
import vocadb.notification.reader.client.util.CommaSeparatedStringToCollectionDeserializer
import vocadb.notification.reader.client.util.ZonedLocalDateTimeDeserializer
import java.time.LocalDateTime
import java.time.ZonedDateTime

data class SongForApiContract constructor(
    val id: Long,
    val additionalNames: String? = null,
    val albums: Collection<AlbumContract>? = null,
    val artists: Collection<ArtistForSongContract>? = null,
    val artistString: String? = null,
    @JsonDeserialize(using = ZonedLocalDateTimeDeserializer::class)
    val createDate: ZonedDateTime? = null,
    val defaultName: String? = null,
    val defaultNameLanguage: Language? = null,
    val deleted: Boolean? = null,
    val favoritedTimes: Int? = null,
    val lengthSeconds: Int? = null,
    val lyrics: Collection<LyricsForSongContract>? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val mergedTo: Int? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringContract>? = null,
    val originalVersionId: Int? = null,
    val publishDate: LocalDateTime? = null,
    val pvs: Collection<PVContract>? = null,
    @JsonDeserialize(using = CommaSeparatedStringToCollectionDeserializer::class)
    val pvServices: Collection<PvServices>? = null,
    val ratingScore: Int? = null,
    val releaseEvent: ReleaseEventForApiContract? = null,
    val songType: SongType,
    val status: Status? = null,
    val tags: Collection<TagUsageForApiContract>? = null,
    val thumbUrl: String? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkForApiContract>? = null
)

data class SongInAlbumForApiContract constructor(
    val discNumber: Int? = null,
    val id: Int? = null,
    val name: String? = null,
    val song: SongForApiContract? = null,
    val trackNumber: Int? = null
)

data class SongListBaseContract constructor(
    val featuredCategory: SongFeaturedCategory? = null,
    val id: Int? = null,
    val name: String? = null
)

enum class SongType(@JsonValue val value: String) {
    UNSPECIFIED("Unspecified"),
    ORIGINAL("Original"),
    REMASTER("Remaster"),
    REMIX("Remix"),
    COVER("Cover"),
    ARRANGEMENT("Arrangement"),
    INSTRUMENTAL("Instrumental"),
    MASHUP("Mashup"),
    MUSIC_PV("MusicPV"),
    DRAMA_PV("DramaPV"),
    LIVE("Live"),
    ILLUSTRATION("Illustration"),
    OTHER("Other");
}

enum class SongFeaturedCategory(@JsonValue val value: String) {
    NOTHING("Nothing"),
    CONCERTS("Concerts"),
    VOCALOID_RANKING("VocaloidRanking"),
    POOLS("Pools"),
    OTHER("Other");
}

enum class PvServices(@JsonValue val value: String) {
    NOTHING("Nothing"),

    NICO_NICO_DOUGA("NicoNicoDouga"),
    YOUTUBE("Youtube"),
    SOUND_CLOUD("SoundCloud"),
    VIMEO("Vimeo"),
    PIAPRO("Piapro"),
    BILIBILI("Bilibili"),
    FILE("File"),
    LOCAL_FILE("LocalFile"),
    CREOFUGA("Creofuga"),
    BANDCAMP("Bandcamp");
}

data class SongListForEditContract constructor(
    val songLinks: Collection<SongInListEditContract>? = null,
    val updateNotes: String? = null,
    val author: UserWithEmailContract? = null,
    val canEdit: Boolean? = null,
    val deleted: Boolean? = null,
    val description: String? = null,
    val eventDate: LocalDateTime? = null,
    val status: Status? = null,
    val thumb: EntryThumbContract? = null,
    val version: Int? = null,
    val featuredCategory: FeaturedCategory? = null,
    val id: Int? = null,
    val name: String? = null
)

data class SongContract constructor(
    val additionalNames: String? = null,
    val artistString: String? = null,
    val createDate: LocalDateTime? = null,
    val deleted: Boolean? = null,
    val favoritedTimes: Int? = null,
    val id: Int? = null,
    val lengthSeconds: Int? = null,
    val name: String? = null,
    val nicoId: String? = null,
    val publishDate: LocalDateTime? = null,
    val pvServices: PvServices? = null,
    val ratingScore: Int? = null,
    val songType: SongType? = null,
    val status: Status? = null,
    val thumbUrl: String? = null,
    val version: Int? = null
)

data class SongRatingContract constructor(
    val rating: SongRating? = null
)

data class SongInListEditContract constructor(
    val songInListId: Int? = null,
    val notes: String? = null,
    val order: Int? = null,
    val song: SongForApiContract? = null
)

data class SongInListForApiContract constructor(
    val notes: String? = null,
    val order: Int? = null,
    val song: SongForApiContract? = null
)

data class SongListForApiContract constructor(
    val author: UserForApiContract? = null,
    val eventDate: LocalDateTime? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val featuredCategory: FeaturedCategory? = null,
    val id: Int? = null,
    val name: String? = null
)

data class RatedSongForUserForApiContract constructor(
    val date: LocalDateTime? = null,
    val song: SongForApiContract? = null,
    val user: UserForApiContract? = null,
    val rating: SongRating? = null
)

enum class SongRating(@JsonValue val value: String) {
    NOTHING("Nothing"),
    DISLIKE("Dislike"),
    LIKE("Like"),
    FAVORITE("Favorite");
}

enum class FeaturedCategory(@JsonValue val value: String) {
    NOTHING("Nothing"),
    CONCERTS("Concerts"),
    VOCALOID_RANKING("VocaloidRanking"),
    POOLS("Pools"),
    OTHER("Other");
}

data class RelatedSongsContract constructor(
    val artistMatches: Collection<SongForApiContract>? = null,
    val likeMatches: Collection<SongForApiContract>? = null,
    val tagMatches: Collection<SongForApiContract>? = null
)
