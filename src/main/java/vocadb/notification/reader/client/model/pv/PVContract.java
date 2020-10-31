package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;

@Getter
@RequiredArgsConstructor
public class PVContract {
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final PvType pvType;
    @JsonProperty
    private final PvService service;
    @JsonProperty
    private final String url;
    @JsonProperty
    private final String name;
    @JsonProperty
    private final Boolean disabled;
    @JsonProperty
    private final @Nullable String author;
    @JsonProperty
    private final @Nullable PVExtendedMetadata extendedMetadata;
    @JsonProperty
    private final @Nullable Integer length;
    @JsonProperty
    private final @Nullable Integer createdBy;
    @JsonProperty
    private final @Nullable LocalDateTime publishDate;
    @JsonProperty
    private final @Nullable String pvId;
    @JsonProperty
    private final @Nullable String thumbUrl;

}
