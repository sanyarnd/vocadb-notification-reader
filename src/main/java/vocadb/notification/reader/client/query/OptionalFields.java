package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum OptionalFields {
    @JsonProperty("None")
    NONE,

    @JsonProperty("Albums")
    ALBUMS,
    @JsonProperty("Artists")
    ARTISTS,
    @JsonProperty("Description")
    DESCRIPTION,
    @JsonProperty("Groups")
    GROUPS,
    @JsonProperty("Lyrics")
    LYRICS,
    @JsonProperty("KnownLanguages")
    KNOWN_LANGUAGES,
    @JsonProperty("MainPicture")
    MAIN_PICTURE,
    @JsonProperty("Members")
    MEMBERS,
    @JsonProperty("Names")
    NAMES,
    @JsonProperty("OldUsernames")
    OLD_USERNAMES,
    @JsonProperty("PVs")
    PVS,
    @JsonProperty("Tags")
    TAGS,
    @JsonProperty("ThumbUrl")
    THUMB_URL,
    @JsonProperty("WebLinks")
    WEB_LINKS;

}
