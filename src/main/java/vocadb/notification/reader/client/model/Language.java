package vocadb.notification.reader.client.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum Language {
    @JsonProperty("Unspecified")
    UNSPECIFIED,
    @JsonProperty("Japanese")
    JAPANESE,
    @JsonProperty("Romaji")
    ROMAJI,
    @JsonProperty("English")
    ENGLISH
}
