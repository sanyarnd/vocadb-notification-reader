package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum FollowedArtistSortRule {
    @JsonProperty("None")
    NONE,
    @JsonProperty("AdditionDate")
    ADDITION_DATE,
    @JsonProperty("AdditionDateAsc")
    ADDITION_DATE_ASC,
    @JsonProperty("FollowerCount")
    FOLLOWER_COUNT,
    @JsonProperty("Name")
    NAME,
    @JsonProperty("ReleaseDate")
    RELEASE_DATE,
    @JsonProperty("SongCount")
    SONG_COUNT,
    @JsonProperty("SongRating")
    SONG_RATING;
}
