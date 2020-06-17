package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class SongListForApiContract {
    @JsonProperty
    private final @Nullable UserForApiContract author;
    @JsonProperty
    private final @Nullable LocalDateTime eventDate;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable FeaturedCategory featuredCategory;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String name;
}
