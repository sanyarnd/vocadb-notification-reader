package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.Status;

@Getter
@RequiredArgsConstructor
public class SongContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable String artistString;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable Integer favoritedTimes;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable Integer lengthSeconds;
    @JsonProperty
    private final @Nullable String name;
    @JsonProperty
    private final @Nullable String nicoId;
    @JsonProperty
    private final @Nullable LocalDateTime publishDate;
    @JsonProperty
    private final @Nullable PvServices pvServices;
    @JsonProperty
    private final @Nullable Integer ratingScore;
    @JsonProperty
    private final @Nullable SongType songType;
    @JsonProperty
    private final @Nullable Status status;
    @JsonProperty
    private final @Nullable String thumbUrl;
    @JsonProperty
    private final @Nullable Integer version;
}
