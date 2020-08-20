package vocadb.notification.reader.model.notification;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.song.SongType;
import vocadb.notification.reader.model.PV;
import vocadb.notification.reader.model.SongNotificationType;
import vocadb.notification.reader.model.Tag;

@Getter
public class SongNotification extends Notification {
    @JsonProperty
    private final SongNotificationType type;
    @JsonProperty
    private final Integer songId;
    @JsonProperty
    private final SongType songType;
    @JsonProperty
    private final @Nullable Collection<Tag> tags;
    @JsonProperty
    private final @Nullable Collection<PV> pvs;
    @JsonProperty
    private final @Nullable String title;
    @JsonProperty
    private final @Nullable String artist;
    @JsonProperty
    private final @Nullable LocalDateTime releaseDate;

    @SuppressWarnings("ParameterNumber")
    public SongNotification(
            Integer id,
            String originalSubject,
            String originalBody,
            SongNotificationType type,
            Integer songId,
            SongType songType,
            @Nullable Collection<Tag> tags,
            @Nullable Collection<PV> pvs,
            @Nullable String title,
            @Nullable String artist,
            @Nullable LocalDateTime releaseDate
    ) {
        super(id, originalSubject, originalBody);
        this.type = type;
        this.songId = songId;
        this.songType = songType;
        this.tags = tags;
        this.pvs = pvs;
        this.title = title;
        this.artist = artist;
        this.releaseDate = releaseDate;
    }
}
