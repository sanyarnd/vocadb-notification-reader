package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class TopSongsVocalist(@JsonValue val value: String) {
    NOTHING("Nothing"),
    VOCALOID("Vocaloid"),
    UTAU("UTAU"),
    CE_VIO("CeVIO")
}
