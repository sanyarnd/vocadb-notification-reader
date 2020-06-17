package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.user.UserForApiContract;

@Getter
@RequiredArgsConstructor
public class AlbumForUserForApiContract {
    @JsonProperty
    private final @Nullable AlbumForApiContract album;
    @JsonProperty
    private final @Nullable AlbumMediaType mediaType;
    @JsonProperty
    private final @Nullable AlbumPurchaseStatus purchaseStatus;
    @JsonProperty
    private final @Nullable Integer rating;
    @JsonProperty
    private final @Nullable UserForApiContract user;

}
