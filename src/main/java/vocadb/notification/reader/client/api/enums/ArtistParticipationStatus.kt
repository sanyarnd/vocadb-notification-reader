package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class ArtistParticipationStatus(@JsonValue val value: String) {
    EVERYTHING("Everything"),
    ONLY_MAIN_ALBUMS("OnlyMainAlbums"),
    ONLY_COLLABORATORS("OnlyCollaborations")
}
