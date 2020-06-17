package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistEventRoles {
    @JsonProperty("Default")
    DEFAULT,
    @JsonProperty("Dancer")
    DANCER,
    @JsonProperty("DJ")
    D_J,
    @JsonProperty("Instrumentalist")
    INSTRUMENTALIST,
    @JsonProperty("Organizer")
    ORGANIZER,
    @JsonProperty("Promoter")
    PROMOTER,
    @JsonProperty("VJ")
    V_J,
    @JsonProperty("Vocalist")
    VOCALIST,
    @JsonProperty("VoiceManipulator")
    VOICE_MANIPULATOR,
    @JsonProperty("OtherPerformer")
    OTHER_PERFORMER,
    @JsonProperty("Other")
    OTHER;
}
