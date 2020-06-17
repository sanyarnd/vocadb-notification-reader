package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.song.SongContract;

@Getter
@RequiredArgsConstructor
public class PVForSongContract {
    @JsonProperty
    private final @Nullable SongContract song;
    @JsonProperty
    private final @Nullable String author;
    @JsonProperty
    private final @Nullable Integer createdBy;
    @JsonProperty
    private final @Nullable Boolean disabled;
    @JsonProperty
    private final @Nullable PVExtendedMetadata extendedMetadata;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable Integer length;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable LocalDateTime publishDate;
    @JsonProperty
    private final @Nullable String pvId;
    @JsonProperty
    private final @Nullable PvService service;
    @JsonProperty
    private final @Nullable PvType pvType;
    @JsonProperty
    private final @Nullable String thumbUrl;
    @JsonProperty
    private final @Nullable String url;
}
