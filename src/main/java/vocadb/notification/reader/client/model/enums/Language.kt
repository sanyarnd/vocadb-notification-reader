package vocadb.notification.reader.client.model.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class Language(@JsonValue val value: String) {
    UNSPECIFIED("Unspecified"),
    JAPANESE("Japanese"),
    ROMAJI("Romaji"),
    ENGLISH("English");
}
