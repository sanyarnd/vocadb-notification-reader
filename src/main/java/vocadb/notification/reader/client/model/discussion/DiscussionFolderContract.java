package vocadb.notification.reader.client.model.discussion;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class DiscussionFolderContract {
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable UserForApiContract lastTopicAuthor;
    @JsonProperty
    private final @Nullable LocalDateTime lastTopicDate;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Integer topicCount;
}
