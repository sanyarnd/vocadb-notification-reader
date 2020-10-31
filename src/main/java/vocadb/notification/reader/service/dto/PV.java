package vocadb.notification.reader.service.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Map;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.pv.PVContract;
import vocadb.notification.reader.client.model.pv.PVExtendedMetadata;
import vocadb.notification.reader.client.model.pv.PvService;
import vocadb.notification.reader.client.model.pv.PvType;

@Getter
@RequiredArgsConstructor
public class PV {
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
    private final @Nullable LocalDateTime publishDate;
    @JsonProperty
    private final @Nullable String pvId;
    @JsonProperty
    private final @Nullable String thumbUrl;
    @JsonProperty
    private final @Nullable String timestamp;

    public static PV of(PVContract pv) {
        PVExtendedMetadata em = pv.extendedMetadata();
        String timestamp = null;
        if (pv.service() == PvService.PIAPRO && em != null) {
            Map<String, String> json = em.json();
            if (json != null) {
                timestamp = json.get("Timestamp");
            }
        }

        return new PV(
                pv.id(),
                pv.pvType(),
                pv.service(),
                pv.url(),
                pv.name(),
                pv.disabled(),
                pv.author(),
                pv.publishDate(),
                pv.pvId(),
                pv.thumbUrl(),
                timestamp
        );
    }
}

