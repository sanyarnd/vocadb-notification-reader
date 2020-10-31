package vocadb.notification.reader.service.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.tag.TagBaseContract;
import vocadb.notification.reader.client.model.tag.TagUsageForApiContract;

@Getter
@RequiredArgsConstructor
public class Tag {
    @JsonProperty
    private final Integer count;
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final String name;
    @JsonProperty
    private final @Nullable String categoryName;

    public static Tag of(TagUsageForApiContract tag) {
        TagBaseContract t = tag.tag();
        return new Tag(tag.count(), t.id(), t.name(), t.categoryName());
    }
}
