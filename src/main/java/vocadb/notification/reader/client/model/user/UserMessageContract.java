package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class UserMessageContract {
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final String subject;
    @JsonProperty
    private final @Nullable String body;
    @JsonProperty
    private final @Nullable String createdFormatted;
    @JsonProperty
    private final @Nullable Boolean highPriority;
    @JsonProperty
    private final @Nullable Inbox inbox;
    @JsonProperty
    private final @Nullable Boolean read;
    @JsonProperty
    private final @Nullable UserForApiContract receiver;
    @JsonProperty
    private final @Nullable UserForApiContract sender;
}
