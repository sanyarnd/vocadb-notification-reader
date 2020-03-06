package vocadb.notification.reader.client.model.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class Status(@JsonValue val value: String) {
    DRAFT("Draft"),
    FINISHED("Finished"),
    APPROVED("Approved"),
    LOCKED("Locked");
}
