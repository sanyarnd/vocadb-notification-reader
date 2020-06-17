package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class LyricsForSongContract {
    @JsonProperty
    private final @Nullable String cultureCode;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String source;
    @JsonProperty
    private final @Nullable TranslationType translationType;
    @JsonProperty
    private final @Nullable String url;
    @JsonProperty
    private final @Nullable String value;
}
