package vocadb.notification.reader.client.model.entrythumb;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Language;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.album.DiscType;
import vocadb.notification.reader.client.model.archived.ArchivedWebLinkContract;
import vocadb.notification.reader.client.model.artist.ArtistType;
import vocadb.notification.reader.client.model.misc.LocalizedStringContract;
import vocadb.notification.reader.client.model.pv.PVContract;
import vocadb.notification.reader.client.model.song.SongFeaturedCategory;
import vocadb.notification.reader.client.model.song.SongType;
import vocadb.notification.reader.client.model.tag.TagUsageForApiContract;

@Getter
@RequiredArgsConstructor
public class EntryForApiContract {
    @JsonProperty
    private final @Nullable LocalDateTime activityDate;
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable String artistString;
    @JsonProperty
    private final @Nullable ArtistType artistType;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable String defaultName;
    @JsonProperty
    private final @Nullable Language defaultNameLanguage;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable DiscType discType;
    @JsonProperty
    private final @Nullable EntryType entryType;
    @JsonProperty
    private final @Nullable EventCategory eventCategory;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringContract> names;
    @JsonProperty
    private final @Nullable Collection<PVContract> pVs;
    @JsonProperty
    private final @Nullable SongFeaturedCategory songListFeaturedCategory;
    @JsonProperty
    private final @Nullable SongType songType;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable String releaseEventSeriesName;
    @JsonProperty
    private final @Nullable String tagCategoryName;
    @JsonProperty
    private final @Nullable Collection<TagUsageForApiContract> tags;
    @JsonProperty
    private final @Nullable String urlSlug;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<ArchivedWebLinkContract> webLinks;
}
