package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum Proficiency {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Basics")
    BASICS,
    @JsonProperty("Intermediate")
    INTERMEDIATE,
    @JsonProperty("Advanced")
    ADVANCED,
    @JsonProperty("Native")
    NATIVE;
}
