package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class ArtistForArtistForApiContract {
    @JsonProperty
    private final @Nullable ArtistContract artist;
    @JsonProperty
    private final @Nullable ArtistLinkType linkType;
}
