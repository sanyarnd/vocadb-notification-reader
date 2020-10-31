package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class AlbumDiscPropertiesContract {
    @JsonProperty
    private final @Nullable Integer discNumber;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable DiscMediaType mediaType;
    @JsonProperty
    private final @Nullable String name;
}
