package vocadb.notification.reader.client.model.album;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class AlbumIdentifierContract {
    @JsonProperty private final @Nullable String value;
}
