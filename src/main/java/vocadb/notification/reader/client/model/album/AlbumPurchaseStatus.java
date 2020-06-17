package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum AlbumPurchaseStatus {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Wishlisted")
    WISHLISTED,
    @JsonProperty("Ordered")
    ORDERED,
    @JsonProperty("Owned")
    OWNED;
}
