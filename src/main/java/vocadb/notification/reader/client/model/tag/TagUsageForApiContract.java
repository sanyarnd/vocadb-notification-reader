package vocadb.notification.reader.client.model.tag;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;

@Getter
@RequiredArgsConstructor
public class TagUsageForApiContract {
    @JsonProperty
    private final TagBaseContract tag;
    @JsonProperty
    private final Integer count;
}
