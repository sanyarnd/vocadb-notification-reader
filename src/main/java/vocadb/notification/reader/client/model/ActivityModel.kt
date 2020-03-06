package vocadb.notification.reader.client.model

import com.fasterxml.jackson.annotation.JsonValue
import java.time.LocalDateTime

data class ActivityEntryForApiContract constructor(
    val archivedVersion: ArchivedObjectVersionForApiContract? = null,
    val author: UserForApiContract? = null,
    val createDate: LocalDateTime? = null,
    val editEvent: ActivityEditEvent? = null,
    val entry: EntryForApiContract? = null
)

enum class ActivityEditEvent(@JsonValue val value: String) {
    CREATED("Created"),
    UPDATED("Updated"),
    DELETED("Deleted"),
    RESTORED("Restored");
}
