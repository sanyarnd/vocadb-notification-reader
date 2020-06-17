package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class SongInListEditContract {
    @JsonProperty
    private final @Nullable Integer songInListId;
    @JsonProperty
    private final @Nullable String notes;
    @JsonProperty
    private final @Nullable Integer order;
    @JsonProperty
    private final @Nullable SongForApiContract song;
}
