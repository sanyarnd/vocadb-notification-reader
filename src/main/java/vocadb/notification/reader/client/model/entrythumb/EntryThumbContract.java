package vocadb.notification.reader.client.model.entrythumb;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class EntryThumbContract {
    @JsonProperty
    private final @Nullable EntryType entryType;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String mime;
    @JsonProperty
    private final @Nullable Integer version;
}

