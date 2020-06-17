package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum GroupId {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Limited")
    LIMITED,
    @JsonProperty("Regular")
    REGULAR,
    @JsonProperty("Trusted")
    TRUSTED,
    @JsonProperty("Moderator")
    MODERATOR,
    @JsonProperty("Admin")
    ADMIN;
}
