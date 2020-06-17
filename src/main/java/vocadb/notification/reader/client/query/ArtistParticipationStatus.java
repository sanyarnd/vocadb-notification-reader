package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistParticipationStatus {
    @JsonProperty("Everything")
    EVERYTHING,
    @JsonProperty("OnlyMainAlbums")
    ONLY_MAIN_ALBUMS,
    @JsonProperty("OnlyCollaborations")
    ONLY_COLLABORATORS;
}
