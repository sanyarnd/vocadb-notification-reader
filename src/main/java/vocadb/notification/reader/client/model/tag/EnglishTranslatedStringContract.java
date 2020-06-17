package vocadb.notification.reader.client.model.tag;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class EnglishTranslatedStringContract {
    @JsonProperty
    private final @Nullable String english;
    @JsonProperty
    private final @Nullable String original;
}
