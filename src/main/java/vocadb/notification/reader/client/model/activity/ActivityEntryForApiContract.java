package vocadb.notification.reader.client.model.activity;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.archived.ArchivedObjectVersionForApiContract;
import vocadb.notification.reader.client.model.entrythumb.EntryForApiContract;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class ActivityEntryForApiContract {
    @JsonProperty
    private final @Nullable ArchivedObjectVersionForApiContract archivedVersion;
    @JsonProperty
    private final @Nullable UserForApiContract author;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable ActivityEditEvent editEvent;
    @JsonProperty
    private final @Nullable EntryForApiContract entry;
}

