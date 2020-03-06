package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.annotation.JsonDeserialize
import com.fasterxml.jackson.module.kotlin.jacksonTypeRef
import java.time.LocalDateTime

data class PVContract constructor(
    val id: Long,
    val pvType: PvType,
    val service: PvService,
    val url: String,
    val name: String,
    val disabled: Boolean,
    val author: String? = null,
    val extendedMetadata: PVExtendedMetadata? = null,
    val length: Long? = null,
    val createdBy: Long? = null,
    val publishDate: LocalDateTime? = null,
    val pvId: String? = null,
    val thumbUrl: String? = null
)

data class PVExtendedMetadata(
    @JsonValue
    @JsonDeserialize(using = RawJsonToMapDeserializer::class)
    val json: Map<String, String?>
)

class RawJsonToMapDeserializer : JsonDeserializer<Map<String, String?>>() {
    override fun deserialize(p: JsonParser, ctxt: DeserializationContext): Map<String, String?> {
        val mapper = p.codec as ObjectMapper
        return mapper.readValue(p.valueAsString, jacksonTypeRef<Map<String, String?>>())
    }
}

enum class PvType(@JsonValue val value: String) {
    ORIGINAL("Original"),
    REPRINT("Reprint"),
    OTHER("Other");
}

enum class PvService(@JsonValue val value: String) {
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

data class PVForSongContract constructor(
    val song: SongContract? = null,
    val author: String? = null,
    val createdBy: Int? = null,
    val disabled: Boolean? = null,
    val extendedMetadata: PVExtendedMetadata? = null,
    val id: Int? = null,
    val length: Int? = null,
    val name: String? = null,
    val publishDate: LocalDateTime? = null,
    val pvId: String? = null,
    val service: PvService? = null,
    val pvType: PvType? = null,
    val thumbUrl: String? = null,
    val url: String? = null
)
