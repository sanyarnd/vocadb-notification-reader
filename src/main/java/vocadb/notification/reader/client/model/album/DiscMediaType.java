package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum DiscMediaType {
    @JsonProperty("Audio")
    AUDIO,
    @JsonProperty("Video")
    VIDEO;
}
