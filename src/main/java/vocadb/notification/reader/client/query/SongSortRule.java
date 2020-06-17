package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum SongSortRule {
    @JsonProperty("None")
    NONE,
    @JsonProperty("Name")
    NAME,
    @JsonProperty("AdditionDate")
    ADDITION_DATE,
    @JsonProperty("FavoritedTimes")
    FAVORITED_TIMES,
    @JsonProperty("RatingScore")
    RATING_SCORE;
}
