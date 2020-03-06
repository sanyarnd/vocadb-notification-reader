package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class LanguagePreference(@JsonValue val value: String) {
    DEFAULT("Default"),
    JAPANESE("Japanese"),
    ROMAJI("Romaji"),
    ENGLISH("English");
}
