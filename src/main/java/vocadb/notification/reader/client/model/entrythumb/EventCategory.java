package vocadb.notification.reader.client.model.entrythumb;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum EventCategory {
    @JsonProperty("Unspecified")
    UNSPECIFIED,
    @JsonProperty("AlbumRelease")
    ALBUM_RELEASE,
    @JsonProperty("Anniversary")
    ANNIVERSARY,
    @JsonProperty("Club")
    CLUB,
    @JsonProperty("Concert")
    CONCERT,
    @JsonProperty("Contest")
    CONTEST,
    @JsonProperty("Convention")
    CONVENTION,
    @JsonProperty("Other")
    OTHER;
}
