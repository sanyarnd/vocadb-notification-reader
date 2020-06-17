package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum SongFeaturedCategory {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Concerts")
    CONCERTS,
    @JsonProperty("VocaloidRanking")
    VOCALOID_RANKING,
    @JsonProperty("Pools")
    POOLS,
    @JsonProperty("Other")
    OTHER;
}
