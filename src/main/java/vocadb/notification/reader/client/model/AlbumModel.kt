package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import vocadb.notification.reader.client.model.enums.Language
import vocadb.notification.reader.client.model.enums.Status
import java.time.LocalDateTime

data class AlbumContract constructor(
    val additionalNames: String? = null,
    val artistString: String? = null,
    val coverPictureMime: String? = null,
    val createDate: LocalDateTime? = null,
    val deleted: Boolean? = null,
    val discType: DiscType? = null,
    val id: Int? = null,
    val name: String? = null,
    val ratingAverage: Double? = null,
    val ratingCount: Int? = null,
    val releaseDate: OptionalDateTimeContract? = null,
    val releaseEvent: ReleaseEventForApiContract? = null,
    val status: Status? = null,
    val version: Int? = null
)

data class AlbumDiscPropertiesContract constructor(
    val discNumber: Int? = null,
    val id: Int? = null,
    val mediaType: DiscMediaType? = null,
    val name: String? = null
)

data class AlbumForApiContract constructor(
    val additionalNames: String? = null,
    val artists: Collection<ArtistForAlbumForApiContract>? = null,
    val artistString: String? = null,
    val barcode: String? = null,
    val catalogNumber: String? = null,
    val createDate: LocalDateTime? = null,
    val defaultName: String? = null,
    val defaultNameLanguage: Language? = null,
    val deleted: Boolean? = null,
    val description: String? = null,
    val discs: Collection<AlbumDiscPropertiesContract>? = null,
    val discType: DiscType? = null,
    val id: Int? = null,
    val identifiers: Collection<AlbumIdentifierContract>? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val mergedTo: Int? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringContract>? = null,
    val pvs: Collection<PVContract>? = null,
    val ratingAverage: Double? = null,
    val ratingCount: Int? = null,
    val releaseDate: OptionalDateTimeContract? = null,
    val releaseEvent: ReleaseEventForApiContract? = null,
    val status: Status? = null,
    val tags: Collection<TagUsageForApiContract>? = null,
    val tracks: Collection<SongInAlbumForApiContract>? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkForApiContract>? = null
)

data class AlbumIdentifierContract constructor(
    val value: String? = null
)

enum class DiscMediaType(@JsonValue val value: String) {
    AUDIO("Audio"),
    VIDEO("Video");
}

enum class DiscType(@JsonValue val value: String) {
    UNKNOWN("Unknown"),
    ALBUM("Album"),
    SINGLE("Single"),
    E_P("EP"),
    SPLIT_ALBUM("SplitAlbum"),
    COMPILATION("Compilation"),
    VIDEO("Video"),
    ARTBOOK("Artbook"),
    GAME("Game"),
    FANMADE("Fanmade"),
    INSTRUMENTAL("Instrumental"),
    OTHER("Other");
}

data class AlbumReviewContract constructor(
    val id: Int? = null,
    val albumId: Int? = null,
    val date: LocalDateTime? = null,
    val languageCode: String? = null,
    val text: String? = null,
    val title: String? = null,
    val user: UserForApiContract? = null
)

data class AlbumForUserForApiContract constructor(
    val album: AlbumForApiContract? = null,
    val mediaType: AlbumMediaType? = null,
    val purchaseStatus: AlbumPurchaseStatus? = null,
    val rating: Int? = null,
    val user: UserForApiContract? = null
)

enum class AlbumPurchaseStatus(@JsonValue val value: String) {
    NOTHING("Nothing"),
    WISHLISTED("Wishlisted"),
    ORDERED("Ordered"),
    OWNED("Owned");
}

enum class AlbumMediaType(@JsonValue val value: String) {
    PHYSICAL_DISC("PhysicalDisc"),
    DIGITAL_DOWNLOAD("DigitalDownload"),
    OTHER("Other");
}
