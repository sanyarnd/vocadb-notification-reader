package vocadb.notification.reader.client.model.archived;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.weblink.WebLinkCategory;

@Getter
@RequiredArgsConstructor
public class ArchivedWebLinkContract {
    @JsonProperty private final @Nullable WebLinkCategory category;
    @JsonProperty private final @Nullable String description;
    @JsonProperty private final @Nullable String url;
}

