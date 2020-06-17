package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.Language;

@Getter
@RequiredArgsConstructor
public class LocalizedStringContract {
    @JsonProperty
    private final @Nullable Language language;
    @JsonProperty
    private final @Nullable String value;
}
