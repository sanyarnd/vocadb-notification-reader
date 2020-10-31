package vocadb.notification.reader.client.model.discussion;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class DiscussionTopicContract {
    @JsonProperty
    private final @Nullable UserForApiContract author;
    @JsonProperty
    private final @Nullable Integer commentCount;
    @JsonProperty
    private final @Nullable Collection<CommentForApiContract> comments;
    @JsonProperty
    private final @Nullable String content;
    @JsonProperty
    private final @Nullable LocalDateTime created;
    @JsonProperty
    private final @Nullable Integer folderId;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable CommentForApiContract lastComment;
    @JsonProperty
    private final @Nullable Boolean locked;
    @JsonProperty
    private final @Nullable String name;
}
