package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class RelatedSongsContract {
    @JsonProperty
    private final @Nullable Collection<SongForApiContract> artistMatches;
    @JsonProperty
    private final @Nullable Collection<SongForApiContract> likeMatches;
    @JsonProperty
    private final @Nullable Collection<SongForApiContract> tagMatches;
}
