package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class FollowedArtistSortRule(@JsonValue val value: String) {
    NONE("None"),

    ADDITION_DATE("AdditionDate"),
    ADDITION_DATE_ASC("AdditionDateAsc"),
    FOLLOWER_COUNT("FollowerCount"),
    NAME("Name"),
    RELEASE_DATE("ReleaseDate"),
    SONG_COUNT("SongCount"),
    SONG_RATING("SongRating");
}
