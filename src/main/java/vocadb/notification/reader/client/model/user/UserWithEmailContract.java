package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class UserWithEmailContract {
    @JsonProperty
    private final @Nullable String email;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
}
