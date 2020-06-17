package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum UserSortRule {
    @JsonProperty("RegisterDate")
    REGISTER_DATE,
    @JsonProperty("Name")
    NAME,
    @JsonProperty("Group")
    GROUP;
}
