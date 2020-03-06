package vocadb.notification.reader.client.api.enums

import com.fasterxml.jackson.annotation.JsonValue

enum class UserSortRule(@JsonValue val value: String) {
    REGISTER_DATE("RegisterDate"),
    NAME("Name"),
    GROUP("Group");
}
