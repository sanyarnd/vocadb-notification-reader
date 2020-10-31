package vocadb.notification.reader.client.model.discussion;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class CommentForApiContract {
    @JsonProperty
    private final @Nullable UserForApiContract author;
    @JsonProperty
    private final @Nullable String authorName;
    @JsonProperty
    private final @Nullable LocalDateTime created;
    @JsonProperty
    private final @Nullable EntryRefContract entry;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String message;
}
