package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum AlbumMediaType {
    @JsonProperty("PhysicalDisc")
    PHYSICAL_DISC,
    @JsonProperty("DigitalDownload")
    DIGITAL_DOWNLOAD,
    @JsonProperty("Other")
    OTHER;
}
