package vocadb.notification.reader.web.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import vocadb.notification.reader.model.notification.Notification;

@Getter
@RequiredArgsConstructor
public class NotificationsDto {
    @JsonProperty
    private final Collection<Notification> notifications;
    @JsonProperty
    private final long totalCount;
}
