package vocadb.notification.reader.client.model.activity;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ActivityEditEvent {
    @JsonProperty("Created")
    CREATED,
    @JsonProperty("Updated")
    UPDATED,
    @JsonProperty("Deleted")
    DELETED,
    @JsonProperty("Restored")
    RESTORED;
}

