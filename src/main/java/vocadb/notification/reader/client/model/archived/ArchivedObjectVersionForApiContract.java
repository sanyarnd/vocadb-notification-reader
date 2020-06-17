package vocadb.notification.reader.client.model.archived;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class ArchivedObjectVersionForApiContract {
    @JsonProperty
    private final @Nullable Collection<String> changedFields;
    @JsonProperty
    private final @Nullable Integer id;
    @JsonProperty
    private final @Nullable String notes;
    @JsonProperty
    private final @Nullable Integer version;
}
