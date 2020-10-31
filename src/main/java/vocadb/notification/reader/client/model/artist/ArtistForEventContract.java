package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class ArtistForEventContract {
    @JsonProperty
    private final @Nullable ArtistContract artist;
    @JsonProperty
    private final @Nullable ArtistEventRoles effectiveRoles;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable ArtistEventRoles roles;
}
