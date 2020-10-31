package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class SongInListForApiContract {
    @JsonProperty
    private final @Nullable String notes;
    @JsonProperty
    private final @Nullable Integer order;
    @JsonProperty
    private final @Nullable SongForApiContract song;
}
