package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum LanguagePreference {
    @JsonProperty("Default")
    DEFAULT,
    @JsonProperty("Japanese")
    JAPANESE,
    @JsonProperty("Romaji")
    ROMAJI,
    @JsonProperty("English")
    ENGLISH;
}
