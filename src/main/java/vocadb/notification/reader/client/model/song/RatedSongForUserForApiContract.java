package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class RatedSongForUserForApiContract {
    @JsonProperty
    private final @Nullable LocalDateTime date;
    @JsonProperty
    private final @Nullable SongForApiContract song;
    @JsonProperty
    private final @Nullable UserForApiContract user;
    @JsonProperty
    private final @Nullable SongRating rating;
}
