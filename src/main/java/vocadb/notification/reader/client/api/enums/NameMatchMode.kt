package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class NameMatchMode(@JsonValue val value: String) {
    AUTO("None"),
    PARTIAL("Partial"),
    STARTS_WIDTH("StartsWidth"),
    EXACT("Exact"),
    WORDS("Words");
}
