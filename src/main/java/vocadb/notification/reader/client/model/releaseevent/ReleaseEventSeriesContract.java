package vocadb.notification.reader.client.model.releaseevent;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.weblink.WebLinkContract;

@Getter
@RequiredArgsConstructor
public class ReleaseEventSeriesContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable ReleaseEventCategory category;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable String pictureMime;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable String urlSlug;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkContract> webLinks;
}
