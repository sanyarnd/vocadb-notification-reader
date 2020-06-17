package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class SongRatingContract {
    @JsonProperty
    private final @Nullable SongRating rating;
}
