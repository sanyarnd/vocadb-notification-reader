package vocadb.notification.reader.client.model.releaseevent;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.artist.ArtistForEventContract;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.misc.LocalizedStringContract;
import vocadb.notification.reader.client.model.song.SongListBaseContract;
import vocadb.notification.reader.client.model.weblink.WebLinkForApiContract;

@Getter
@RequiredArgsConstructor
public class ReleaseEventForApiContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable Collection<ArtistForEventContract> artists;
    @JsonProperty
    private final @Nullable ReleaseEventCategory category;
    @JsonProperty
    private final @Nullable LocalDateTime date;
    @JsonProperty
    private final @Nullable String description;
    @JsonProperty
    private final @Nullable LocalDateTime endDate;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringContract> names;
    @JsonProperty
    private final @Nullable ReleaseEventSeriesContract series;
    @JsonProperty
    private final @Nullable Integer seriesId;
    @JsonProperty
    private final @Nullable Integer seriesNumber;
    @JsonProperty
    private final @Nullable String seriesSuffix;
    @JsonProperty
    private final @Nullable SongListBaseContract songList;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable String urlSlug;
    @JsonProperty
    private final @Nullable String venueName;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkForApiContract> webLinks;
}
