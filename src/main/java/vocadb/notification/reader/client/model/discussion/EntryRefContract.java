package vocadb.notification.reader.client.model.discussion;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.entrythumb.EntryType;

@Getter
@RequiredArgsConstructor
public class EntryRefContract {
    @JsonProperty
    private final @Nullable EntryType entryType;
    @JsonProperty
    private final @Nullable Integer id;
}
