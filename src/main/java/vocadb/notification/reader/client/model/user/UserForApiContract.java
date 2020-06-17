package vocadb.notification.reader.client.model.user;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.time.LocalDateTime;
import java.util.Collection;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.entrythumb.EntryThumbForApiContract;
import vocadb.notification.reader.client.model.misc.OldUsernameContract;

@Getter
@RequiredArgsConstructor
public class UserForApiContract {
    @JsonProperty
    private final Integer id;
    @JsonProperty
    private final String name;
    @JsonProperty
    private final @Nullable Boolean active;
    @JsonProperty
    private final @Nullable GroupId groupId;
    @JsonProperty
    private final @Nullable Collection<UserKnownLanguageContract> knownLanguages;
    @JsonProperty
    private final @Nullable EntryThumbForApiContract mainPicture;
    @JsonProperty
    private final @Nullable LocalDateTime memberSince;
    @JsonProperty
    private final @Nullable Collection<OldUsernameContract> oldUsernames;
    @JsonProperty
    private final @Nullable Boolean verifiedArtist;
}
