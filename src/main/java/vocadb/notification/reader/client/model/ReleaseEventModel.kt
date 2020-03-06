package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import vocadb.notification.reader.client.model.enums.Status
import java.time.LocalDateTime

data class ReleaseEventForApiContract constructor(
    val additionalNames: String? = null,
    val artists: Collection<ArtistForEventContract>? = null,
    val category: ReleaseEventCategory? = null,
    val date: LocalDateTime? = null,
    val description: String? = null,
    val endDate: LocalDateTime? = null,
    val id: Int? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringContract>? = null,
    val series: ReleaseEventSeriesContract? = null,
    val seriesId: Int? = null,
    val seriesNumber: Int? = null,
    val seriesSuffix: String? = null,
    val songList: SongListBaseContract? = null,
    val status: Status? = null,
    val urlSlug: String? = null,
    val venueName: String? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkForApiContract>? = null
)

data class ReleaseEventSeriesContract constructor(
    val additionalNames: String? = null,
    val category: ReleaseEventCategory? = null,
    val deleted: Boolean? = null,
    val description: String? = null,
    val id: Int? = null,
    val name: String? = null,
    val pictureMime: String? = null,
    val status: Status? = null,
    val urlSlug: String? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkContract>? = null
)

enum class ReleaseEventCategory(@JsonValue val value: String) {
    UNSPECIFIED("Unspecified"),
    ALBUM_RELEASE("AlbumRelease"),
    ANNIVERSARY("Anniversary"),
    CLUB("Club"),
    CONCERT("Concert"),
    CONTEST("Contest"),
    CONVENTION("Convention"),
    OTHER("Other");
}
