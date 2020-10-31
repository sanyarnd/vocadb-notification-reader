package vocadb.notification.reader.client.model.misc;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class PartialFindResult<T> {
    @JsonProperty
    private final @Nullable Collection<T> items;
    @JsonProperty
    private final @Nullable Integer totalCount;
    @JsonProperty
    private final @Nullable String term;
}
