package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import vocadb.notification.reader.client.model.enums.Language
import java.time.LocalDateTime

data class LocalizedStringContract constructor(
    val language: Language? = null,
    val value: String? = null
)

data class LyricsForSongContract constructor(
    val cultureCode: String? = null,
    val id: Int? = null,
    val source: String? = null,
    val translationType: TranslationType? = null,
    val url: String? = null,
    val value: String? = null
)

data class OldUsernameContract constructor(
    val date: LocalDateTime? = null,
    val oldName: String? = null
)

data class OptionalDateTimeContract constructor(
    val day: Int? = null,
    val formatted: String? = null,
    val isEmpty: Boolean? = null,
    val month: Int? = null,
    val year: Int? = null
)

data class PartialFindResult<T> constructor(
    val items: Collection<T>? = null,
    val totalCount: Long? = null,
    val term: String? = null
)

enum class TranslationType(@JsonValue val value: String) {
    ORIGINAL("Original"),
    ROMANIZED("Romanized"),
    TRANSLATION("Translation");
}

data class AdvancedSearchFilter constructor(
    val filterType: FilterType? = null,
    val negate: Boolean? = null,
    val param: String? = null
)

enum class FilterType(@JsonValue val value: String) {
    NOTHING("Nothing"),
    ARTIST_TYPE("ArtistType"),
    WEB_LINK("WebLink"),
    HAS_USER_ACCOUNT("HasUserAccount"),
    ROOT_VOICEBANK("RootVoicebank"),
    VOICE_PROVIDER("VoiceProvider"),
    HAS_STORE_LINK("HasStoreLink"),
    HAS_TRACKS("HasTracks"),
    NO_COVER_PICTURE("NoCoverPicture"),
    HAS_ALBUM("HasAlbum"),
    HAS_ORIGINAL_MEDIA("HasOriginalMedia"),
    HAS_MEDIA("HasMedia"),
    HAS_MULTIPLE_VOICEBANKS("HasMultipleVoicebanks"),
    HAS_PUBLISH_DATE("HasPublishDate"),
    LYRICS("Lyrics"),
    LYRICS_CONTENT("LyricsContent");
}
