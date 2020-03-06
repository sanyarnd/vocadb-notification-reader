package vocadb.notification.reader.client.api

import com.fasterxml.jackson.databind.ObjectMapper
import mu.KotlinLogging
import vocadb.notification.reader.client.api.enums.ArtistParticipationStatus
import vocadb.notification.reader.client.api.enums.LanguagePreference
import vocadb.notification.reader.client.api.enums.NameMatchMode
import vocadb.notification.reader.client.api.enums.OptionalFields
import vocadb.notification.reader.client.api.enums.SongSortRule
import vocadb.notification.reader.client.api.enums.TopSongsFilterRule
import vocadb.notification.reader.client.api.enums.TopSongsVocalist
import vocadb.notification.reader.client.model.LyricsForSongContract
import vocadb.notification.reader.client.model.PartialFindResult
import vocadb.notification.reader.client.model.PvService
import vocadb.notification.reader.client.model.PvServices
import vocadb.notification.reader.client.model.RatedSongForUserForApiContract
import vocadb.notification.reader.client.model.SongForApiContract
import vocadb.notification.reader.client.model.SongType
import vocadb.notification.reader.client.model.enums.Status
import java.net.URI
import java.net.http.HttpClient
import java.time.LocalDateTime
import java.util.concurrent.CompletableFuture

private val logger = KotlinLogging.logger { }

class VocaDbSongApi(
    client: HttpClient,
    baseUrl: URI,
    objectMapper: ObjectMapper
) : BaseApi(client, baseUrl, objectMapper) {

    /**
     * Gets a song by Id.
     *
     * @param id Song Id (required).
     * @param fields List of optional fields. Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang Content language preference. (optional)
     */
    fun getSongById(
        id: Long,
        fields: Collection<OptionalFields>? = null,
        lang: LanguagePreference? = null
    ): CompletableFuture<SongForApiContract> {
        logger.debug { "Fetching song(id=$id) info" }
        val params: Map<String, Any?> = mapOf(
            "fields" to fields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/songs/$id?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets a song by PV.
     *
     * @param pvService PV service (required). Possible values are NicoNicoDouga, Youtube, SoundCloud, Vimeo, Piapro, Bilibili.
     * @param pvId PV Id (required). For example sm123456.
     * @param fields List of optional fields. Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang Content language preference. (optional)
     */

    fun getSongByPv(
        pvService: PvService,
        pvId: String,
        fields: Collection<OptionalFields>? = null,
        lang: LanguagePreference? = null
    ): CompletableFuture<SongForApiContract> {
        val params: Map<String, Any?> = mapOf(
            "pvService" to pvService,
            "pvId" to pvId,
            "fields" to fields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/songs/byPv?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets list of highlighted songs, same as front page.
     * Output is cached for 1 hour.
     * @param language  (optional)
     * @param fields  (optional)
     */

    fun getHighlightedSongs(
        language: LanguagePreference? = null,
        fields: Collection<OptionalFields>? = null
    ): CompletableFuture<Collection<SongForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "language" to language,
            "fields" to fields
        )

        val request = baseUrl.resolve("/api/songs/highlighted?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Find songs.
     *
     * @param query Song name query. (optional)
     * @param songTypes Filtered song types. Possible values are Original, Remaster, Remix, Cover, Instrumental, Mashup, MusicPV, DramaPV, Other. (optional)
     * @param afterDate Filter by songs published after this date (inclusive). (optional)
     * @param beforeDate Filter by songs published before this date (exclusive). (optional)
     * @param tagName Filter by one or more tag names. (optional)
     * @param tagId Filter by one or more tag Ids. (optional)
     * @param childTags Include child tags, if the tags being filtered by have any. (optional)
     * @param artistId Filter by artist Id. (optional)
     * @param artistParticipationStatus Filter by artist participation status. Only valid if artistId is specified. Everything (default): Show all songs by that artist (no filter).              OnlyMainAlbums: Show only main songs by that artist. OnlyCollaborations: Show only collaborations by that artist. (optional)
     * @param childVoicebanks Include child voicebanks, if the artist being filtered by has any. (optional)
     * @param includeMembers Include members of groups. This applies if {artistId} is a group. (optional)
     * @param onlyWithPvs Whether to only include songs with at least one PV. (optional)
     * @param pvServices Filter by one or more PV services (separated by commas). The song will pass the filter if it has a PV for any of the matched services. (optional)
     * @param since Allow only entries that have been created at most this many hours ago. By default there is no filtering. (optional)
     * @param minScore Minimum rating score. Optional. (optional)
     * @param userCollectionId Filter by user&#x27;s rated songs. By default there is no filtering. (optional)
     * @param releaseEventId Filter by release event. By default there is no filtering. (optional)
     * @param status Filter by entry status. (optional)
     * @param advancedFilters List of advanced filters. (optional)
     * @param start First item to be retrieved (optional, defaults to 0). (optional)
     * @param maxResults Maximum number of results to be loaded (optional, defaults to 10, maximum of 50). (optional)
     * @param getTotalCount Whether to load total number of items (optional, default to false). (optional)
     * @param sort Sort rule (optional, defaults to Name). Possible values are None, Name, AdditionDate, FavoritedTimes, RatingScore. (optional)
     * @param preferAccurateMatches Whether the search should prefer accurate matches.               If this is true, entries that match by prefix will be moved first, instead of being sorted alphabetically.              Requires a text query. Does not support pagination.              This is mostly useful for autocomplete boxes. (optional)
     * @param nameMatchMode Match mode for song name (optional, defaults to Exact). (optional)
     * @param fields List of optional fields. Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang Content language preference. (optional)
     */

    fun findSongs(
        query: String? = null,
        songTypes: Collection<SongType>? = null,
        afterDate: LocalDateTime? = null,
        beforeDate: LocalDateTime? = null,
        tagName: Collection<String>? = null,
        tagId: Collection<Int>? = null,
        childTags: Boolean? = null,
        artistId: Collection<Int>? = null,
        artistParticipationStatus: ArtistParticipationStatus? = null,
        childVoicebanks: Boolean? = null,
        includeMembers: Boolean? = null,
        onlyWithPvs: Boolean? = null,
        pvServices: Collection<PvServices>? = null,
        since: Int? = null,
        minScore: Int? = null,
        userCollectionId: Int? = null,
        releaseEventId: Int? = null,
        status: Status? = null,
        advancedFilters: Collection<Any>? = null,
        start: Int? = null,
        maxResults: Int? = null,
        getTotalCount: Boolean? = null,
        sort: SongSortRule? = null,
        preferAccurateMatches: Boolean? = null,
        nameMatchMode: NameMatchMode? = null,
        fields: Collection<OptionalFields>? = null,
        lang: LanguagePreference? = null
    ): CompletableFuture<PartialFindResult<SongForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "query" to query,
            "songTypes" to songTypes,
            "afterDate" to afterDate,
            "beforeDate" to beforeDate,
            "tagName" to tagName,
            "tagId" to tagId,
            "childTags" to childTags,
            "artistId" to artistId,
            "artistParticipationStatus" to artistParticipationStatus,
            "childVoicebanks" to childVoicebanks,
            "includeMembers" to includeMembers,
            "onlyWithPvs" to onlyWithPvs,
            "pvServices" to pvServices,
            "since" to since,
            "minScore" to minScore,
            "userCollectionId" to userCollectionId,
            "releaseEventId" to releaseEventId,
            "status" to status,
            "advancedFilters" to advancedFilters,
            "start" to start,
            "maxResults" to maxResults,
            "getTotalCount" to getTotalCount,
            "sort" to sort,
            "preferAccurateMatches" to preferAccurateMatches,
            "nameMatchMode" to nameMatchMode,
            "fields" to fields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/songs?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets lyrics by ID.
     * Output is cached. Specify song version as parameter to refresh.
     * @param lyricsId Lyrics ID.
     */
    fun getLyrics(lyricsId: Int): CompletableFuture<LyricsForSongContract> {
        val request = baseUrl.resolve("/api/songs/lyrics/$lyricsId").getRequest()
        return sendAsync(request)
    }

    /**
     * Get ratings for a song.
     * The result includes ratings and user information. For users who have requested not to make their ratings public, the user will be empty.
     * @param id Song ID.
     * @param userFields Optional fields for the users.
     * @param lang Content language preference. (optional)
     */

    fun getSongRatings(
        id: Int,
        userFields: Collection<OptionalFields> = listOf(OptionalFields.NONE),
        lang: LanguagePreference? = null
    ): CompletableFuture<Collection<RatedSongForUserForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "userFields" to userFields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/songs/$id/ratings?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets top rated songs.
     *
     * @param durationHours Duration in hours from which to get songs. (optional)
     * @param startDate Lower bound of the date. Optional. (optional)
     * @param filterBy Filtering mode. (optional)
     * @param vocalist Vocalist selection. (optional)
     * @param maxResults Maximum number of results to be loaded. (optional)
     * @param fields Optional song fields to load. (optional)
     * @param languagePreference Language preference. (optional)
     */

    fun getTopSongs(
        durationHours: Int? = null,
        startDate: LocalDateTime? = null,
        filterBy: TopSongsFilterRule? = null,
        vocalist: TopSongsVocalist? = null,
        maxResults: Int? = null,
        fields: Collection<OptionalFields>? = null,
        languagePreference: LanguagePreference? = null
    ): CompletableFuture<Collection<SongForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "durationHours" to durationHours,
            "startDate" to startDate,
            "filterBy" to filterBy,
            "vocalist" to vocalist,
            "maxResults" to maxResults,
            "fields" to fields,
            "languagePreference" to languagePreference
        )

        val request = baseUrl.resolve("/api/songs/top-rated?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets derived (alternate versions) of a song.
     * Pagination and sorting might be added later.
     * @param id Song Id (required).
     * @param fields List of optional fields. Possible values are Albums, Artists, Names, PVs, Tags, ThumbUrl, WebLinks. (optional)
     * @param lang Content language preference. (optional)
     */

    fun getDerivedSongs(
        id: Int,
        fields: Collection<OptionalFields>? = null,
        lang: LanguagePreference? = null
    ): CompletableFuture<Collection<SongForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "fields" to fields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/songs/$id/derived?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

}
