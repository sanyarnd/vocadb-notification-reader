package vocadb.notification.reader.web.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Getter;
import lombok.RequiredArgsConstructor;

@Getter
@RequiredArgsConstructor
public class AccountDto {
    @JsonProperty
    private final int vocadbId;
    @JsonProperty
    private final String username;
}
