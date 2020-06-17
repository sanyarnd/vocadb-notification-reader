package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.Map;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
@RequiredArgsConstructor
public class PVExtendedMetadata {
    @JsonProperty
    @JsonDeserialize(using = RawJsonToMapDeserializer.class)
    private final Map<String, @Nullable String> json;
}
