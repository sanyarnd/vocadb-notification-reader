package vocadb.notification.reader.client.model.artist;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.album.AlbumForApiContract;
import vocadb.notification.reader.client.model.releaseevent.ReleaseEventForApiContract;
import vocadb.notification.reader.client.model.song.SongForApiContract;

@Getter
@RequiredArgsConstructor
public class ArtistRelationsForApi {
    @JsonProperty
    private final @Nullable Collection<AlbumForApiContract> latestAlbums;
    @JsonProperty
    private final @Nullable Collection<ReleaseEventForApiContract> latestEvents;
    @JsonProperty
    private final @Nullable Collection<SongForApiContract> latestSongs;
    @JsonProperty
    private final @Nullable Collection<AlbumForApiContract> popularAlbums;
    @JsonProperty
    private final @Nullable Collection<SongForApiContract> popularSongs;
}
