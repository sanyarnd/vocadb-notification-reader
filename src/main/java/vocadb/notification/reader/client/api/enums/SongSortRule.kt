package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class SongSortRule(@JsonValue val value: String) {
    NONE("None"),
    NAME("Name"),
    ADDITION_DATE("AdditionDate"),
    FAVORITED_TIMES("FavoritedTimes"),
    RATING_SCORE("RatingScore")
}
