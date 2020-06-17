package vocadb.notification.reader.client.model.weblink;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum WebLinkCategory {
    @JsonProperty("Official")
    OFFICIAL,
    @JsonProperty("Commercial")
    COMMERCIAL,
    @JsonProperty("Reference")
    REFERENCE,
    @JsonProperty("Other")
    OTHER;
}
