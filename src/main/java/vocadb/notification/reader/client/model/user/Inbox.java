package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum Inbox {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Received")
    RECEIVED,
    @JsonProperty("Sent")
    SENT,
    @JsonProperty("Notifications")
    NOTIFICATIONS;
}
