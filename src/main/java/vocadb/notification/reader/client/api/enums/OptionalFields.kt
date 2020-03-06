package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class OptionalFields(@JsonValue val value: String) {
    NONE("None"),

    ALBUMS("Albums"),
    ARTISTS("Artists"),
    DESCRIPTION("Description"),
    GROUPS("Groups"),
    KNOWN_LANGUAGES("KnownLanguages"),
    MAIN_PICTURE("MainPicture"),
    MEMBERS("Members"),
    NAMES("Names"),
    OLD_USERNAMES("OldUsernames"),
    PVS("PVs"),
    TAGS("Tags"),
    THUMB_URL("ThumbUrl"),
    WEB_LINKS("WebLinks");
}
