package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistCategories {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Vocalist")
    VOCALIST,
    @JsonProperty("Producer")
    PRODUCER,
    @JsonProperty("Animator")
    ANIMATOR,
    @JsonProperty("Label")
    LABEL,
    @JsonProperty("Circle")
    CIRCLE,
    @JsonProperty("Other")
    OTHER,
    @JsonProperty("Band")
    BAND,
    @JsonProperty("Illustrator")
    ILLUSTRATOR,
    @JsonProperty("Subject")
    SUBJECT;
}
