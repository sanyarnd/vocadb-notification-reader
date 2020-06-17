package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum NameMatchMode {
    @JsonProperty("None")
    AUTO,
    @JsonProperty("Partial")
    PARTIAL,
    @JsonProperty("StartsWidth")
    STARTS_WIDTH,
    @JsonProperty("Exact")
    EXACT,
    @JsonProperty("Words")
    WORDS;
}
