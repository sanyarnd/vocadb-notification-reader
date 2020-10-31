package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class ArtistForArtistForApiContract {
    @JsonProperty
    private final @Nullable ArtistContract artist;
    @JsonProperty
    private final @Nullable ArtistLinkType linkType;
}
