package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum PvService {
    @JsonProperty("NicoNicoDouga")
    NICO_NICO_DOUGA,
    @JsonProperty("Youtube")
    YOUTUBE,
    @JsonProperty("SoundCloud")
    SOUND_CLOUD,
    @JsonProperty("Vimeo")
    VIMEO,
    @JsonProperty("Piapro")
    PIAPRO,
    @JsonProperty("Bilibili")
    BILIBILI,
    @JsonProperty("File")
    FILE,
    @JsonProperty("LocalFile")
    LOCAL_FILE,
    @JsonProperty("Creofuga")
    CREOFUGA,
    @JsonProperty("Bandcamp")
    BANDCAMP;
}
