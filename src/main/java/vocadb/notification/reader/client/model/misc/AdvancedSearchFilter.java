package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class AdvancedSearchFilter {
    @JsonProperty
    private final @Nullable FilterType filterType;
    @JsonProperty
    private final @Nullable Boolean negate;
    @JsonProperty
    private final @Nullable String param;
}
