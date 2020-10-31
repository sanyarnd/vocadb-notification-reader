package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class ArtistForAlbumForApiContract {
    @JsonProperty
    private final @Nullable ArtistContract artist;
    @JsonProperty
    private final @Nullable ArtistCategories categories;
    @JsonProperty
    private final @Nullable ArtistRoles effectiveRoles;
    @JsonProperty
    private final @Nullable Boolean isSupport;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable ArtistRoles roles;

}
