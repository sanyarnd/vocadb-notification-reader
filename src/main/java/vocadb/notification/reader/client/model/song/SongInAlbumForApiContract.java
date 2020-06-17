package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class SongInAlbumForApiContract {
    @JsonProperty
    private final @Nullable Integer discNumber;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable SongForApiContract song;
    @JsonProperty
    private final @Nullable Integer trackNumber;
}
