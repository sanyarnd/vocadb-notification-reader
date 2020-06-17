package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum ArtistLinkType {
    @JsonProperty("CharacterDesigner")
    CHARACTER_DESIGNER,
    @JsonProperty("Group")
    GROUP,
    @JsonProperty("Illustrator")
    ILLUSTRATOR,
    @JsonProperty("Manager")
    MANAGER,
    @JsonProperty("VoiceProvider")
    VOICE_PROVIDER;
}
