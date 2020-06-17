package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum TopSongsFilterRule {
    @JsonProperty("CreateDate")
    CREATE_DATE,
    @JsonProperty("PublishDate")
    PUBLISH_DATE,
    @JsonProperty("Popularity")
    POPULARITY;
}
