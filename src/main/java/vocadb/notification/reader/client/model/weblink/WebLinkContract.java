package vocadb.notification.reader.client.model.weblink;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class WebLinkContract {
    @JsonProperty
    private final @Nullable WebLinkCategory category;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable String descriptionOrUrl;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String url;
}
