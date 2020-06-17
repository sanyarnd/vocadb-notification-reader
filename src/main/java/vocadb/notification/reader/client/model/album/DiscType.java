package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum DiscType {
    @JsonProperty("Unknown")
    UNKNOWN,
    @JsonProperty("Album")
    ALBUM,
    @JsonProperty("Single")
    SINGLE,
    @JsonProperty("EP")
    E_P,
    @JsonProperty("SplitAlbum")
    SPLIT_ALBUM,
    @JsonProperty("Compilation")
    COMPILATION,
    @JsonProperty("Video")
    VIDEO,
    @JsonProperty("Artbook")
    ARTBOOK,
    @JsonProperty("Game")
    GAME,
    @JsonProperty("Fanmade")
    FANMADE,
    @JsonProperty("Instrumental")
    INSTRUMENTAL,
    @JsonProperty("Other")
    OTHER;
}
