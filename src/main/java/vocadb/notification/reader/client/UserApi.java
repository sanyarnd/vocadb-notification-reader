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
import org.checkerframework.checker.nullness.qual.Nullable;
import vocadb.notification.reader.client.model.misc.PartialFindResult;
import vocadb.notification.reader.client.model.user.Inbox;
import vocadb.notification.reader.client.model.user.UserForApiContract;
import vocadb.notification.reader.client.model.user.UserMessageContract;
import vocadb.notification.reader.client.query.OptionalFields;

public class UserApi extends Api {
    public UserApi(HttpClient client, URI baseUrl, ObjectMapper objectMapper) {
        super(client, objectMapper, baseUrl);
    }

    /**
     * Get current user data.
     */
    public CompletionStage<UserForApiContract> getCurrentUser(@Nullable Collection<OptionalFields> fields) {
        List<Pair<String, ?>> params = List.of(
                Pair.of("fields", fields)
        );
        HttpRequest request = HttpUtils.getRequest(baseUrl.resolve(String.format("/api/users/current?%s", toQuery(params))));
        return sendAsync(request, new TypeReference<>() {});
    }

    /**
     * Gets a list of messages.
     *
     * @param userId        User ID. Must be the currently logged in user (loading messages for another user is not allowed).
     * @param inbox         Type of inbox.
     *                      Possible values are Nothing (load all, default), Received, Sent, Notifications. (optional)
     * @param unread        Whether to only load unread messages.
     *                      Loading unread messages is only possible for received messages and notifications (not sent messages). (optional)
     * @param anotherUserId Filter by id of the other user (either sender or receiver). (optional)
     * @param start         Index of the first entry to be loaded. (optional)
     * @param maxResults    Maximum number of results to be loaded. (optional)
     * @param getTotalCount Whether to get total number of results. (optional)
     */

    public CompletionStage<PartialFindResult<UserMessageContract>> getMessages(
            int userId,
            @Nullable Inbox inbox,
            @Nullable Boolean unread,
            @Nullable Integer anotherUserId,
            @Nullable Integer start,
            @Nullable Integer maxResults,
            @Nullable Boolean getTotalCount
    ) {
        List<Pair<String, ?>> params = List.of(
                Pair.of("inbox", inbox),
                Pair.of("unread", unread),
                Pair.of("anotherUserId", anotherUserId),
                Pair.of("start", start),
                Pair.of("maxResults", maxResults),
                Pair.of("getTotalCount", getTotalCount)
        );
        HttpRequest request = HttpUtils.getRequest(baseUrl.resolve(String.format("/api/users/%d/messages?%s", userId, toQuery(params))));
        return sendAsync(request, new TypeReference<>() {});
    }

    /**
     * Gets a user message.
     * The message will be marked as read. User can only load messages from their own inbox.
     *
     * @param messageId ID of the message.
     */
    public CompletionStage<UserMessageContract> getMessage(int messageId) {
        HttpRequest request = HttpUtils.getRequest(baseUrl.resolve(String.format("/api/users/messages/%d", messageId)));
        return sendAsync(request, new TypeReference<>() {});
    }

    /**
     * Deletes a list of user messages.
     *
     * @param userId     ID of the user whose messages to delete.
     * @param messageIds IDs of messages.
     */
    public CompletionStage<Void> deleteMessages(int userId, Collection<Integer> messageIds) {
        if (messageIds.isEmpty()) {
            throw new IllegalArgumentException("Need at least 1 message id");
        }
        List<Pair<String, ?>> params = List.of(
                Pair.of("messageId", messageIds)
        );
        HttpRequest request = HttpUtils.deleteRequest(baseUrl.resolve(String.format("/api/users/%d/messages?%s", userId, toQuery(params))));
        return sendAsync(request);
    }

}
