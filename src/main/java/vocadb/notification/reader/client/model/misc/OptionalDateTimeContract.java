package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class OptionalDateTimeContract {
    @JsonProperty
    private final @Nullable Integer day;
    @JsonProperty
    private final @Nullable String formatted;
    @JsonProperty
    private final @Nullable Boolean isEmpty;
    @JsonProperty
    private final @Nullable Integer month;
    @JsonProperty
    private final @Nullable Integer year;
}
