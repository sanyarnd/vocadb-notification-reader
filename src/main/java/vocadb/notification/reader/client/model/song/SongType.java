package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum SongType {
    @JsonProperty("Unspecified")
    UNSPECIFIED,
    @JsonProperty("Original")
    ORIGINAL,
    @JsonProperty("Remaster")
    REMASTER,
    @JsonProperty("Remix")
    REMIX,
    @JsonProperty("Cover")
    COVER,
    @JsonProperty("Arrangement")
    ARRANGEMENT,
    @JsonProperty("Instrumental")
    INSTRUMENTAL,
    @JsonProperty("Mashup")
    MASHUP,
    @JsonProperty("MusicPV")
    MUSIC_PV,
    @JsonProperty("DramaPV")
    DRAMA_PV,
    @JsonProperty("Live")
    LIVE,
    @JsonProperty("Illustration")
    ILLUSTRATION,
    @JsonProperty("Other")
    OTHER;
}
