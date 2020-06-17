package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum TranslationType {
    @JsonProperty("Original")
    ORIGINAL,
    @JsonProperty("Romanized")
    ROMANIZED,
    @JsonProperty("Translation")
    TRANSLATION;
}
