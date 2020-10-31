package vocadb.notification.reader.client.model.tag;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class TagBaseContract {
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final String name;
    @JsonProperty
    private final @Nullable String categoryName;
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable String urlSlug;
}
