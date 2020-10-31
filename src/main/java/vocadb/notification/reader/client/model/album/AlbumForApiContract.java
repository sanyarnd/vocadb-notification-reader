package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Language;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.artist.ArtistForAlbumForApiContract;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.misc.LocalizedStringContract;
import vocadb.notification.reader.client.model.misc.OptionalDateTimeContract;
import vocadb.notification.reader.client.model.pv.PVContract;
import vocadb.notification.reader.client.model.releaseevent.ReleaseEventForApiContract;
import vocadb.notification.reader.client.model.song.SongInAlbumForApiContract;
import vocadb.notification.reader.client.model.tag.TagUsageForApiContract;
import vocadb.notification.reader.client.model.weblink.WebLinkForApiContract;

@Getter
@RequiredArgsConstructor
public class AlbumForApiContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable Collection<ArtistForAlbumForApiContract> artists;
    @JsonProperty
    private final @Nullable String artistString;
    @JsonProperty
    private final @Nullable String barcode;
    @JsonProperty
    private final @Nullable String catalogNumber;
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
    private final @Nullable Collection<AlbumDiscPropertiesContract> discs;
    @JsonProperty
    private final @Nullable DiscType discType;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable Collection<AlbumIdentifierContract> identifiers;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable Integer mergedTo;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringContract> names;
    @JsonProperty
    private final @Nullable Collection<PVContract> pvs;
    @JsonProperty
    private final @Nullable Double ratingAverage;
    @JsonProperty
    private final @Nullable Integer ratingCount;
    @JsonProperty
    private final @Nullable OptionalDateTimeContract releaseDate;
    @JsonProperty
    private final @Nullable ReleaseEventForApiContract releaseEvent;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable Collection<TagUsageForApiContract> tags;
    @JsonProperty
    private final @Nullable Collection<SongInAlbumForApiContract> tracks;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkForApiContract> webLinks;
}
