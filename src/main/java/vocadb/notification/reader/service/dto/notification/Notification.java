package vocadb.notification.reader.service.dto.notification;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import lombok.RequiredArgsConstructor;

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "notificationType")
@JsonSubTypes({
        @JsonSubTypes.Type(value = AlbumNotification.class, name = "AlbumNotification"),
        @JsonSubTypes.Type(value = ArtistNotification.class, name = "ArtistNotification"),
        @JsonSubTypes.Type(value = EventNotification.class, name = "EventNotification"),
        @JsonSubTypes.Type(value = ReportNotification.class, name = "ReportNotification"),
        @JsonSubTypes.Type(value = SongNotification.class, name = "SongNotification"),
        @JsonSubTypes.Type(value = UnknownNotification.class, name = "UnknownNotification")
})
@RequiredArgsConstructor
public abstract class Notification {
    @JsonProperty
    protected final Integer id;
    @JsonProperty
    protected final String originalSubject;
    @JsonProperty
    protected final String originalBody;
}
