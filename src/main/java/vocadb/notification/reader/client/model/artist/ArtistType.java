package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistType {
    @JsonProperty("Unknown")
    UNKNOWN,
    @JsonProperty("Circle")
    CIRCLE,
    @JsonProperty("Label")
    LABEL,
    @JsonProperty("Producer")
    PRODUCER,
    @JsonProperty("Animator")
    ANIMATOR,
    @JsonProperty("Illustrator")
    ILLUSTRATOR,
    @JsonProperty("Lyricist")
    LYRICIST,
    @JsonProperty("Vocaloid")
    VOCALOID,
    @JsonProperty("UTAU")
    U_TAU,
    @JsonProperty("CeVIO")
    CE_VIO,
    @JsonProperty("OtherVoiceSynthesizer")
    OTHER_VOICE_SYNTHESIZER,
    @JsonProperty("OtherVocalist")
    OTHER_VOCALIST,
    @JsonProperty("OtherGroup")
    OTHER_GROUP,
    @JsonProperty("OtherIndividual")
    OTHER_INDIVIDUAL,
    @JsonProperty("Utaite")
    UTAITE,
    @JsonProperty("Band")
    BAND,
    @JsonProperty("Vocalist")
    VOCALIST,
    @JsonProperty("Character")
    CHARACTER;
}
