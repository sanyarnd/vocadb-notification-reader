package vocadb.notification.reader.client.model;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum Status {
    @JsonProperty("Draft")
    DRAFT,
    @JsonProperty("Finished")
    FINISHED,
    @JsonProperty("Approved")
    APPROVED,
    @JsonProperty("Locked")
    LOCKED;
}
