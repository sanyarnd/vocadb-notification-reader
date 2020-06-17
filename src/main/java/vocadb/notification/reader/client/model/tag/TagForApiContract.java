package vocadb.notification.reader.client.model.tag;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.Language;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.weblink.WebLinkForApiContract;

@Getter
@RequiredArgsConstructor
public class TagForApiContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable TagBaseContract aliasedTo;
    @JsonProperty
    private final @Nullable String categoryName;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable Language defaultNameLanguage;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringWithIdContract> names;
    @JsonProperty
    private final @Nullable TagBaseContract parent;
    @JsonProperty
    private final @Nullable Collection<TagBaseContract> relatedTags;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable Integer targets;
    @JsonProperty
    private final @Nullable EnglishTranslatedStringContract translatedDescription;
    @JsonProperty
    private final @Nullable String urlSlug;
    @JsonProperty
    private final @Nullable Integer usageCount;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkForApiContract> webLinks;
}
