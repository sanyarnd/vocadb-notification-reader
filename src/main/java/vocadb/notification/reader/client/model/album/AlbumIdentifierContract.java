package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class AlbumIdentifierContract {
    @JsonProperty private final @Nullable String value;
}
