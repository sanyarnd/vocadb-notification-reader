package vocadb.notification.reader.client.api

import com.fasterxml.jackson.databind.ObjectMapper
import mu.KotlinLogging
import vocadb.notification.reader.client.api.enums.FollowedArtistSortRule
import vocadb.notification.reader.client.api.enums.LanguagePreference
import vocadb.notification.reader.client.api.enums.NameMatchMode
import vocadb.notification.reader.client.api.enums.OptionalFields
import vocadb.notification.reader.client.api.enums.UserSortRule
import vocadb.notification.reader.client.model.ArtistForUserForApiContract
import vocadb.notification.reader.client.model.ArtistType
import vocadb.notification.reader.client.model.GroupId
import vocadb.notification.reader.client.model.Inbox
import vocadb.notification.reader.client.model.PartialFindResult
import vocadb.notification.reader.client.model.UserForApiContract
import vocadb.notification.reader.client.model.UserMessageContract
import java.net.URI
import java.net.http.HttpClient
import java.time.LocalDateTime
import java.util.concurrent.CompletableFuture

private val logger = KotlinLogging.logger { }

class VocaDbUserApi(
    client: HttpClient,
    baseUrl: URI,
    objectMapper: ObjectMapper
) : BaseApi(client, baseUrl, objectMapper) {

    /**
     * Get current user data.
     */
    fun getCurrentUser(fields: Collection<OptionalFields>? = null): CompletableFuture<UserForApiContract> {
        val params: Map<String, Any?> = mapOf(
            "fields" to fields
        )

        val request = baseUrl.resolve("/api/users/current?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets a list of users.
     *
     * @param query User name query. (optional)
     * @param groups Filter by user group. Only one value supported for now. (optional)
     * @param joinDateAfter Filter by users who joined after this date (inclusive). (optional)
     * @param joinDateBefore Filter by users who joined before this date (exclusive). (optional)
     * @param nameMatchMode Name match mode. (optional)
     * @param start Index of the first entry to be loaded. (optional)
     * @param maxResults Maximum number of results to be loaded. (optional)
     * @param getTotalCount Whether to get total number of results. (optional)
     * @param sort Sort rule. (optional)
     * @param includeDisabled Whether to include disabled user accounts. (optional)
     * @param onlyVerified Whether to only include verified artists. (optional)
     * @param knowsLanguage Filter by known language. This is the ISO 639-1 language code, for example \&quot;en\&quot; or \&quot;zh\&quot;. (optional)
     * @param fields Optional fields. Possible values are None and MainPicture. (optional)
     */

    fun getUsers(
        query: String? = null,
        groups: GroupId? = null,
        joinDateAfter: LocalDateTime? = null,
        joinDateBefore: LocalDateTime? = null,
        nameMatchMode: NameMatchMode? = null,
        start: Int? = null,
        maxResults: Int? = null,
        getTotalCount: Boolean? = null,
        sort: UserSortRule? = null,
        includeDisabled: Boolean? = null,
        onlyVerified: Boolean? = null,
        knowsLanguage: String? = null,
        fields: Collection<OptionalFields>? = null
    ): CompletableFuture<PartialFindResult<UserForApiContract>> {

        val params: Map<String, Any?> = mapOf(
            "query" to query,
            "groups" to groups,
            "joinDateAfter" to joinDateAfter,
            "joinDateBefore" to joinDateBefore,
            "nameMatchMode" to nameMatchMode,
            "start" to start,
            "maxResults" to maxResults,
            "getTotalCount" to getTotalCount,
            "sort" to sort,
            "includeDisabled" to includeDisabled,
            "onlyVerified" to onlyVerified,
            "knowsLanguage" to knowsLanguage,
            "fields" to fields
        )

        val request = baseUrl.resolve("/api/users?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets a list of messages.
     *
     * @param id User ID. Must be the currently logged in user (loading messages for another user is not allowed).
     * @param inbox Type of inbox. Possible values are Nothing (load all, default), Received, Sent, Notifications. (optional)
     * @param unread Whether to only load unread messages. Loading unread messages is only possible for received messages and notifications (not sent messages). (optional)
     * @param anotherUserId Filter by id of the other user (either sender or receiver). (optional)
     * @param start Index of the first entry to be loaded. (optional)
     * @param maxResults Maximum number of results to be loaded. (optional)
     * @param getTotalCount Whether to get total number of results. (optional)
     */

    fun getMessages(
        id: Long,
        inbox: Inbox? = null,
        unread: Boolean? = null,
        anotherUserId: Int? = null,
        start: Long? = null,
        maxResults: Long? = null,
        getTotalCount: Boolean? = null
    ): CompletableFuture<PartialFindResult<UserMessageContract>> {
        val params: Map<String, Any?> = mapOf(
            "inbox" to inbox,
            "unread" to unread,
            "anotherUserId" to anotherUserId,
            "start" to start,
            "maxResults" to maxResults,
            "getTotalCount" to getTotalCount
        )

        val request = baseUrl.resolve("/api/users/$id/messages?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

    /**
     * Gets a user message.
     * The message will be marked as read. User can only load messages from their own inbox.
     * @param messageId ID of the message.
     */
    fun getMessage(messageId: Long): CompletableFuture<UserMessageContract> {
        logger.debug { "Fetching message id=$messageId" }
        val request = baseUrl.resolve("/api/users/messages/$messageId").getRequest()
        return sendAsync(request)
    }

    /**
     * Deletes a list of user messages.
     *
     * @param id ID of the user whose messages to delete.
     * @param messageIds IDs of messages.
     */
    fun deleteMessages(id: Long, messageIds: Collection<Long>): CompletableFuture<Void> {
        require(messageIds.isNotEmpty())
        logger.debug { "Deleting messages $messageIds for userId=$id" }
        val params: String = messageIds.joinToString("&") { messageId -> "messageId=$messageId" }

        val request = baseUrl.resolve("/api/users/$id/messages?${params}").deleteRequest()
        return sendAsyncVoid(request)
    }

    /**
     * Gets a list of artists followed by a user.
     *
     * @param id ID of the user whose followed artists are to be browsed.
     * @param query Artist name query. (optional)
     * @param artistType Filter by artist type. (optional)
     * @param start First item to be retrieved (defaults to 0). (optional)
     * @param maxResults Maximum number of results to be loaded (optional, defaults to 10, maximum of 50). (optional)
     * @param getTotalCount Whether to load total number of items (optional, default to false). (optional)
     * @param sort Sort rule (optional, defaults to Name). Possible values are None, Name, AdditionDate, AdditionDateAsc. (optional)
     * @param nameMatchMode Match mode for artist name (optional, defaults to Auto). (optional)
     * @param fields List of optional fields. Possible values are Description, Groups, Members, Names, Tags, WebLinks. (optional)
     * @param lang Content language preference. (optional)
     */

    fun getFollowedArtists(
        id: Int,
        query: String? = null,
        artistType: ArtistType? = null,
        start: Int? = null,
        maxResults: Int? = null,
        getTotalCount: Boolean? = null,
        sort: FollowedArtistSortRule? = null,
        nameMatchMode: NameMatchMode? = null,
        fields: Collection<OptionalFields>? = null,
        lang: LanguagePreference? = null
    ): CompletableFuture<PartialFindResult<ArtistForUserForApiContract>> {
        val params: Map<String, Any?> = mapOf(
            "query" to query,
            "artistType" to artistType,
            "start" to start,
            "maxResults" to maxResults,
            "getTotalCount" to getTotalCount,
            "sort" to sort,
            "nameMatchMode" to nameMatchMode,
            "fields" to fields,
            "lang" to lang
        )

        val request = baseUrl.resolve("/api/users/$id/followedArtists?${params.toQuery()}").getRequest()
        return sendAsync(request)
    }

}
