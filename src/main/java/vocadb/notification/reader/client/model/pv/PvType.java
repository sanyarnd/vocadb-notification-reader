package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum PvType {
    @JsonProperty("Original")
    ORIGINAL,
    @JsonProperty("Reprint")
    REPRINT,
    @JsonProperty("Other")
    OTHER;
}
