package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class OldUsernameContract {
    @JsonProperty
    private final @Nullable LocalDateTime date;
    @JsonProperty
    private final @Nullable String oldName;
}
