package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class AlbumReviewContract {
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable Integer albumId;
    @JsonProperty
    private final @Nullable LocalDateTime date;
    @JsonProperty
    private final @Nullable String languageCode;
    @JsonProperty
    private final @Nullable String text;
    @JsonProperty
    private final @Nullable String title;
    @JsonProperty
    private final @Nullable UserForApiContract user;
}
