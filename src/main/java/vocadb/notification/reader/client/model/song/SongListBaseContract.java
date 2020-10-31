package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class SongListBaseContract {
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable SongFeaturedCategory featuredCategory;
    @JsonProperty
    private final @Nullable String name;
}
