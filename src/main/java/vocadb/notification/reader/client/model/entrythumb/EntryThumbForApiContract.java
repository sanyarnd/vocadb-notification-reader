package vocadb.notification.reader.client.model.entrythumb;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class EntryThumbForApiContract {
    @JsonProperty
    private final @Nullable String mime;
    @JsonProperty
    private final @Nullable String urlSmallThumb;
    @JsonProperty
    private final @Nullable String urlThumb;
    @JsonProperty
    private final @Nullable String urlTinyThumb;
}
