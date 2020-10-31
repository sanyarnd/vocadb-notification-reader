package vocadb.notification.reader.client.model.tag;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Language;

@Getter
@RequiredArgsConstructor
public class LocalizedStringWithIdContract {
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable Language language;
    @JsonProperty
    private final @Nullable String value;
}
