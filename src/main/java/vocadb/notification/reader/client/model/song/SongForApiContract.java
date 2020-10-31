package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.time.LocalDateTime;
import java.time.ZonedDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Language;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.album.AlbumContract;
import vocadb.notification.reader.client.model.artist.ArtistForSongContract;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.misc.LocalizedStringContract;
import vocadb.notification.reader.client.model.misc.LyricsForSongContract;
import vocadb.notification.reader.client.model.pv.PVContract;
import vocadb.notification.reader.client.model.releaseevent.ReleaseEventForApiContract;
import vocadb.notification.reader.client.model.tag.TagUsageForApiContract;
import vocadb.notification.reader.client.model.weblink.WebLinkForApiContract;

@Getter
@RequiredArgsConstructor
public class SongForApiContract {
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable Collection<AlbumContract> albums;
    @JsonProperty
    private final @Nullable Collection<ArtistForSongContract> artists;
    @JsonProperty
    private final @Nullable String artistString;
    @JsonProperty
    @JsonDeserialize(using = ZonedLocalDateTimeDeserializer.class)
    private final @Nullable ZonedDateTime createDate;
    @JsonProperty
    private final @Nullable String defaultName;
    @JsonProperty
    private final @Nullable Language defaultNameLanguage;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable Integer favoritedTimes;
    @JsonProperty
    private final @Nullable Integer lengthSeconds;
    @JsonProperty
    private final @Nullable Collection<LyricsForSongContract> lyrics;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable Integer mergedTo;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable Collection<LocalizedStringContract> names;
    @JsonProperty
    private final @Nullable Integer originalVersionId;
    @JsonProperty
    private final @Nullable LocalDateTime publishDate;
    @JsonProperty
    private final @Nullable Collection<PVContract> pvs;
    @JsonProperty
    @JsonDeserialize(using = CommaSeparatedStringToCollectionDeserializer.class)
    private final @Nullable Collection<PvServices> pvServices;
    @JsonProperty
    private final @Nullable Integer ratingScore;
    @JsonProperty
    private final @Nullable ReleaseEventForApiContract releaseEvent;
    @JsonProperty
    private final SongType songType;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable Collection<TagUsageForApiContract> tags;
    @JsonProperty
    private final @Nullable String thumbUrl;
    @JsonProperty
    private final @Nullable Integer version;
    @JsonProperty
    private final @Nullable Collection<WebLinkForApiContract> webLinks;
}
