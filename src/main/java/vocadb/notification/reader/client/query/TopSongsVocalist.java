package vocadb.notification.reader.client.query;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum TopSongsVocalist {
    @JsonProperty("Nothing")
    NOTHING,
    @JsonProperty("Vocaloid")
    VOCALOID,
    @JsonProperty("UTAU")
    UTAU,
    @JsonProperty("CeVIO")
    CE_VIO;
}
