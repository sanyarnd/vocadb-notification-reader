package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import vocadb.notification.reader.client.model.enums.Language
import vocadb.notification.reader.client.model.enums.Status
import java.time.LocalDateTime

data class ArtistContract constructor(
    val additionalNames: String? = null,
    val artistType: ArtistType? = null,
    val deleted: Boolean? = null,
    val id: Int? = null,
    val name: String? = null,
    val pictureMime: String? = null,
    val releaseDate: LocalDateTime? = null,
    val status: Status? = null,
    val version: Int? = null
)

data class ArtistForAlbumForApiContract constructor(
    val artist: ArtistContract? = null,
    val categories: ArtistCategories? = null,
    val effectiveRoles: ArtistRoles? = null,
    val isSupport: Boolean? = null,
    val name: String? = null,
    val roles: ArtistRoles? = null
)

data class ArtistForApiContract constructor(
    val additionalNames: String? = null,
    val artistLinks: Collection<ArtistForArtistForApiContract>? = null,
    val artistLinksReverse: Collection<ArtistForArtistForApiContract>? = null,
    val artistType: ArtistType? = null,
    val baseVoicebank: ArtistContract? = null,
    val createDate: LocalDateTime? = null,
    val defaultName: String? = null,
    val defaultNameLanguage: Language? = null,
    val deleted: Boolean? = null,
    val description: String? = null,
    val id: Int? = null,
    val mainPicture: EntryThumbForApiContract? = null,
    val mergedTo: Int? = null,
    val name: String? = null,
    val names: Collection<LocalizedStringContract>? = null,
    val pictureMime: String? = null,
    val relations: ArtistRelationsForApi? = null,
    val releaseDate: LocalDateTime? = null,
    val status: Status? = null,
    val tags: Collection<TagUsageForApiContract>? = null,
    val version: Int? = null,
    val webLinks: Collection<WebLinkForApiContract>? = null
)

data class ArtistForArtistForApiContract constructor(
    val artist: ArtistContract? = null,
    val linkType: ArtistLinkType? = null
)

data class ArtistForEventContract constructor(
    val artist: ArtistContract? = null,
    val effectiveRoles: ArtistEventRoles? = null,
    val id: Int? = null,
    val name: String? = null,
    val roles: ArtistEventRoles? = null
)

data class ArtistForSongContract constructor(
    val artist: ArtistContract? = null,
    val categories: ArtistCategories? = null,
    val effectiveRoles: ArtistRoles? = null,
    val id: Int? = null,
    val isCustomName: Boolean? = null,
    val isSupport: Boolean? = null,
    val name: String? = null,
    val roles: ArtistRoles? = null
)

data class ArtistForUserForApiContract constructor(
    val artist: ArtistForApiContract? = null
)

data class ArtistRelationsForApi constructor(
    val latestAlbums: Collection<AlbumForApiContract>? = null,
    val latestEvents: Collection<ReleaseEventForApiContract>? = null,
    val latestSongs: Collection<SongForApiContract>? = null,
    val popularAlbums: Collection<AlbumForApiContract>? = null,
    val popularSongs: Collection<SongForApiContract>? = null
)

enum class ArtistCategories(@JsonValue val value: String) {
    NOTHING("Nothing"),
    VOCALIST("Vocalist"),
    PRODUCER("Producer"),
    ANIMATOR("Animator"),
    LABEL("Label"),
    CIRCLE("Circle"),
    OTHER("Other"),
    BAND("Band"),
    ILLUSTRATOR("Illustrator"),
    SUBJECT("Subject");
}

enum class ArtistEventRoles(@JsonValue val value: String) {
    DEFAULT("Default"),
    DANCER("Dancer"),
    D_J("DJ"),
    INSTRUMENTALIST("Instrumentalist"),
    ORGANIZER("Organizer"),
    PROMOTER("Promoter"),
    V_J("VJ"),
    VOCALIST("Vocalist"),
    VOICE_MANIPULATOR("VoiceManipulator"),
    OTHER_PERFORMER("OtherPerformer"),
    OTHER("Other");
}

enum class ArtistLinkType(@JsonValue val value: String) {
    CHARACTER_DESIGNER("CharacterDesigner"),
    GROUP("Group"),
    ILLUSTRATOR("Illustrator"),
    MANAGER("Manager"),
    VOICE_PROVIDER("VoiceProvider");
}

enum class ArtistRoles(@JsonValue val value: String) {
    DEFAULT("Default"),
    ANIMATOR("Animator"),
    ARRANGER("Arranger"),
    COMPOSER("Composer"),
    DISTRIBUTOR("Distributor"),
    ILLUSTRATOR("Illustrator"),
    INSTRUMENTALIST("Instrumentalist"),
    LYRICIST("Lyricist"),
    MASTERING("Mastering"),
    PUBLISHER("Publisher"),
    VOCALIST("Vocalist"),
    VOICE_MANIPULATOR("VoiceManipulator"),
    OTHER("Other"),
    MIXER("Mixer"),
    CHORUS("Chorus"),
    ENCODER("Encoder"),
    VOCAL_DATA_PROVIDER("VocalDataProvider");
}

enum class ArtistType(@JsonValue val value: String) {
    UNKNOWN("Unknown"),
    CIRCLE("Circle"),
    LABEL("Label"),
    PRODUCER("Producer"),
    ANIMATOR("Animator"),
    ILLUSTRATOR("Illustrator"),
    LYRICIST("Lyricist"),
    VOCALOID("Vocaloid"),
    U_TAU("UTAU"),
    CE_VIO("CeVIO"),
    OTHER_VOICE_SYNTHESIZER("OtherVoiceSynthesizer"),
    OTHER_VOCALIST("OtherVocalist"),
    OTHER_GROUP("OtherGroup"),
    OTHER_INDIVIDUAL("OtherIndividual"),
    UTAITE("Utaite"),
    BAND("Band"),
    VOCALIST("Vocalist"),
    CHARACTER("Character");
}
