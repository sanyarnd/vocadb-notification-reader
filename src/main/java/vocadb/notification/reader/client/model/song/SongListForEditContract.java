package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbContract;
import vocadb.notification.reader.client.model.user.UserWithEmailContract;

@Getter
@RequiredArgsConstructor
public class SongListForEditContract {
    @JsonProperty
    private final @Nullable Collection<SongInListEditContract> songLinks;
    @JsonProperty
    private final @Nullable String updateNotes;
    @JsonProperty
    private final @Nullable UserWithEmailContract author;
    @JsonProperty
    private final @Nullable Boolean canEdit;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable LocalDateTime eventDate;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable EntryThumbContract thumb;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable FeaturedCategory featuredCategory;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
}
