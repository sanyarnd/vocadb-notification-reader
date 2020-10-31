package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Language;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.misc.LocalizedStringContract;
import vocadb.notification.reader.client.model.tag.TagUsageForApiContract;
import vocadb.notification.reader.client.model.weblink.WebLinkForApiContract;

@Getter
@RequiredArgsConstructor
public class ArtistForApiContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable Collection<ArtistForArtistForApiContract> artistLinks;
    @JsonProperty
    private final @Nullable Collection<ArtistForArtistForApiContract> artistLinksReverse;
    @JsonProperty
    private final @Nullable ArtistType artistType;
    @JsonProperty
    private final @Nullable ArtistContract baseVoicebank;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable String defaultName;
    @JsonProperty
    private final @Nullable Language defaultNameLanguage;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable Integer mergedTo;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringContract> names;
    @JsonProperty
    private final @Nullable String pictureMime;
    @JsonProperty
    private final @Nullable ArtistRelationsForApi relations;
    @JsonProperty
    private final @Nullable LocalDateTime releaseDate;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable Collection<TagUsageForApiContract> tags;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkForApiContract> webLinks;
}
