package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class TopSongsFilterRule(@JsonValue val value: String) {
    CREATE_DATE("CreateDate"),
    PUBLISH_DATE("PublishDate"),
    POPULARITY("Popularity")
}
