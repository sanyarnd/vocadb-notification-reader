package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistRoles {
    @JsonProperty("Default")
    DEFAULT,
    @JsonProperty("Animator")
    ANIMATOR,
    @JsonProperty("Arranger")
    ARRANGER,
    @JsonProperty("Composer")
    COMPOSER,
    @JsonProperty("Distributor")
    DISTRIBUTOR,
    @JsonProperty("Illustrator")
    ILLUSTRATOR,
    @JsonProperty("Instrumentalist")
    INSTRUMENTALIST,
    @JsonProperty("Lyricist")
    LYRICIST,
    @JsonProperty("Mastering")
    MASTERING,
    @JsonProperty("Publisher")
    PUBLISHER,
    @JsonProperty("Vocalist")
    VOCALIST,
    @JsonProperty("VoiceManipulator")
    VOICE_MANIPULATOR,
    @JsonProperty("Other")
    OTHER,
    @JsonProperty("Mixer")
    MIXER,
    @JsonProperty("Chorus")
    CHORUS,
    @JsonProperty("Encoder")
    ENCODER,
    @JsonProperty("VocalDataProvider")
    VOCAL_DATA_PROVIDER;
}
