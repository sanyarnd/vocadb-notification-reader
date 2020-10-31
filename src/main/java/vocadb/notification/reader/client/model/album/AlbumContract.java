package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.Status;
import vocadb.notification.reader.client.model.misc.OptionalDateTimeContract;
import vocadb.notification.reader.client.model.releaseevent.ReleaseEventForApiContract;

@Getter
@RequiredArgsConstructor
public class AlbumContract {
    @JsonProperty
    private final @Nullable String additionalNames;
    @JsonProperty
    private final @Nullable String artistString;
    @JsonProperty
    private final @Nullable String coverPictureMime;
    @JsonProperty
    private final @Nullable LocalDateTime createDate;
    @JsonProperty
    private final @Nullable Boolean deleted;
    @JsonProperty
    private final @Nullable DiscType discType;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
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
    private final @Nullable Integer version;
}

