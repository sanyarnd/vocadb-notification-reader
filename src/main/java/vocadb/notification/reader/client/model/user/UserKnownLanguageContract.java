package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class UserKnownLanguageContract {
    @JsonProperty
    private final @Nullable String cultureCode;
    @JsonProperty
    private final @Nullable Proficiency proficiency;
}
