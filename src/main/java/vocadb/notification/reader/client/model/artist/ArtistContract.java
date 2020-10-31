package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Status;

@Getter
@RequiredArgsConstructor
public class ArtistContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable ArtistType artistType;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable String pictureMime;
    @JsonProperty
    private final @Nullable LocalDateTime releaseDate;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable Integer version;
}
