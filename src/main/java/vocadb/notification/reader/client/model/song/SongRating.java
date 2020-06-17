package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum SongRating {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Dislike")
    DISLIKE,
    @JsonProperty("Like")
    LIKE,
    @JsonProperty("Favorite")
    FAVORITE;
}
