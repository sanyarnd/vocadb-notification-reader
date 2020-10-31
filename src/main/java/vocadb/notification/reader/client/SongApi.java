package vocadb.notification.reader.client;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.util.Collection;
import java.util.List;
import java.util.concurrent.CompletionStage;
import org.apache.commons.lang3.tuple.Pair;
import org.jetbrains.annotations.Nullable;
import vocadb.notification.reader.client.model.pv.PvService;
import vocadb.notification.reader.client.model.song.SongForApiContract;
import vocadb.notification.reader.client.query.LanguagePreference;
import vocadb.notification.reader.client.query.OptionalFields;

@SuppressWarnings("MultipleStringLiterals")
public class SongApi extends Api {
    public SongApi(HttpClient client, URI baseUrl, ObjectMapper objectMapper) {
        super(client, objectMapper, baseUrl);
    }

    /**
     * Gets a song by Id.
     *
     * @param id     Song Id (required).
     * @param fields List of optional fields.
     *               Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang   Content language preference. (optional)
     */
    public CompletionStage<SongForApiContract> getSongById(
            int id,
            @Nullable Collection<OptionalFields> fields,
            @Nullable LanguagePreference lang
    ) {
        List<Pair<String, ?>> params = List.of(
                Pair.of("fields", fields),
                Pair.of("lang", lang)
        );

        HttpRequest request =
                HttpUtils.getRequest(baseUrl.resolve(String.format("/api/songs/%d?%s", id, toQuery(params))));
        return sendAsync(request, new TypeReference<>() {});
    }

    /**
     * Gets a song by PV.
     *
     * @param pvService PV service (required).
     *                  Possible values are NicoNicoDouga, Youtube, SoundCloud, Vimeo, Piapro, Bilibili.
     * @param pvId      PV Id (required). For example sm123456.
     * @param fields    List of optional fields.
     *                  Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang      Content language preference. (optional)
     */

    public CompletionStage<SongForApiContract> getSongByPv(
            PvService pvService,
            String pvId,
            @Nullable Collection<OptionalFields> fields,
            @Nullable LanguagePreference lang
    ) {
        List<Pair<String, ?>> params = List.of(
                Pair.of("pvService", pvService),
                Pair.of("pvId", pvId),
                Pair.of("fields", fields),
                Pair.of("lang", lang)
        );

        HttpRequest request =
                HttpUtils.getRequest(baseUrl.resolve(String.format("/api/songs/byPv?%s", toQuery(params))));
        return sendAsync(request, new TypeReference<>() {});
    }

    /**
     * Gets list of highlighted songs, same as front page.
     * Output is cached for 1 hour.
     *
     * @param language (optional)
     * @param fields   (optional)
     */
    public CompletionStage<Collection<SongForApiContract>> getHighlightedSongs(
            @Nullable LanguagePreference language,
            @Nullable Collection<OptionalFields> fields
    ) {
        List<Pair<String, ?>> params = List.of(
                Pair.of("language", language),
                Pair.of("fields", fields)
        );
        HttpRequest request =
                HttpUtils.getRequest(baseUrl.resolve(String.format("/api/songs/highlighted?%s", toQuery(params))));
        return sendAsync(request, new TypeReference<>() {});
    }
}
