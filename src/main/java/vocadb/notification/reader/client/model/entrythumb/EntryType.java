package vocadb.notification.reader.client.model.entrythumb;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum EntryType {
    @JsonProperty("Undefined")
    UNDEFINED,
    @JsonProperty("Album")
    ALBUM,
    @JsonProperty("Artist")
    ARTIST,
    @JsonProperty("DiscussionTopic")
    DISCUSSION_TOPIC,
    @JsonProperty("PV")
    PV,
    @JsonProperty("ReleaseEvent")
    RELEASE_EVENT,
    @JsonProperty("ReleaseEventSeries")
    RELEASE_EVENT_SERIES,
    @JsonProperty("Song")
    SONG,
    @JsonProperty("SongList")
    SONG_LIST,
    @JsonProperty("Tag")
    TAG,
    @JsonProperty("User")
    USER;
}
