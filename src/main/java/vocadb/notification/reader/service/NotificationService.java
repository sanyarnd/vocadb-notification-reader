package vocadb.notification.reader.service;

import java.time.LocalDateTime;
import java.util.Collection;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import lombok.RequiredArgsConstructor;
import org.apache.commons.lang3.tuple.Pair;
import org.springframework.beans.factory.ObjectProvider;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.client.VocaDbClient;
import vocadb.notification.reader.client.model.song.SongType;
import vocadb.notification.reader.client.model.user.Inbox;
import vocadb.notification.reader.client.model.user.UserMessageContract;
import vocadb.notification.reader.client.query.LanguagePreference;
import vocadb.notification.reader.client.query.OptionalFields;
import vocadb.notification.reader.service.dto.NotificationType;
import vocadb.notification.reader.service.dto.PV;
import vocadb.notification.reader.service.dto.SongNotificationType;
import vocadb.notification.reader.service.dto.Tag;
import vocadb.notification.reader.service.dto.notification.AlbumNotification;
import vocadb.notification.reader.service.dto.notification.ArtistNotification;
import vocadb.notification.reader.service.dto.notification.EventNotification;
import vocadb.notification.reader.service.dto.notification.Notification;
import vocadb.notification.reader.service.dto.notification.ReportNotification;
import vocadb.notification.reader.service.dto.notification.SongNotification;
import vocadb.notification.reader.service.dto.notification.UnknownNotification;
import static java.util.Collections.emptyList;
import static vocadb.notification.reader.service.dto.NotificationType.UNKNOWN;

@Component
@RequiredArgsConstructor
public class NotificationService {
    private final ObjectProvider<VocaDbClient> clientProvider;

    public Mono<Void> deleteNotifications(
            int userId,
            Collection<Integer> notificationIds,
            List<String> authCookies
    ) {
        var client = clientProvider.getObject();
        client.setCookies(authCookies);
        return Mono.fromCompletionStage(client.userApi().deleteMessages(userId, notificationIds));
    }

    public CompletableFuture<Pair<Collection<Notification>, Integer>> loadNotifications(
            int userId,
            int startOffset,
            int maxResults,
            List<String> authCookies,
            LanguagePreference languagePreference
    ) {
        var client = clientProvider.getObject();
        client.setCookies(authCookies);

        return client.userApi()
                .getMessages(userId, Inbox.NOTIFICATIONS, null, null, startOffset, maxResults, true)
                .toCompletableFuture()
                .thenApply(r -> {
                    Integer totalCount = r.totalCount();
                    if (r.items() == null) {
                        return Pair.of(emptyList(), 0);
                    }

                    List<Notification> msgStream = Stream.ofNullable(r.items()).flatMap(Collection::stream)
                            .map(msg -> client.userApi().getMessage(msg.id()).toCompletableFuture())
                            .map(msg -> toNotification(msg, client, languagePreference))
                            .map(CompletableFuture::join)
                            .collect(Collectors.toList());

                    return Pair.of(msgStream, totalCount);
                });
    }

    private CompletableFuture<? extends Notification> toNotification(
            CompletableFuture<UserMessageContract> message,
            VocaDbClient client,
            LanguagePreference languagePreference
    ) {
        return message.thenComposeAsync(msg -> {
            Pair<NotificationType, Integer> detection = detectNotificationType(msg);
            NotificationType type = detection.getLeft();
            Integer payloadId = detection.getRight();

            String originalBody = msg.body();
            String body = originalBody == null ? "<empty body>" : originalBody;

            CompletableFuture<? extends Notification> notification;
            switch (type) {
                case SONG:
                    notification = createSongNotification(
                            client,
                            msg.id(),
                            msg.subject(),
                            body,
                            payloadId,
                            languagePreference
                    );
                    break;
                case ARTIST:
                    notification =
                            CompletableFuture.completedFuture(new ArtistNotification(msg.id(), msg.subject(), body));
                    break;
                case EVENT:
                    notification =
                            CompletableFuture.completedFuture(new EventNotification(msg.id(), msg.subject(), body));
                    break;
                case ALBUM:
                    notification =
                            CompletableFuture.completedFuture(new AlbumNotification(msg.id(), msg.subject(), body));
                    break;
                case REPORT:
                    notification =
                            CompletableFuture.completedFuture(new ReportNotification(msg.id(), msg.subject(), body));
                    break;
                default:
                    notification =
                            CompletableFuture.completedFuture(new UnknownNotification(msg.id(), msg.subject(), body));
                    break;
            }

            return notification;
        });
    }

    private CompletableFuture<SongNotification> createSongNotification(
            VocaDbClient client,
            int id,
            String subject,
            String body,
            int songId,
            LanguagePreference languagePreference
    ) {
        return client.songApi()
                .getSongById(songId, List.of(OptionalFields.TAGS, OptionalFields.PVS), languagePreference)
                .toCompletableFuture()
                .thenApplyAsync(song -> {
                    SongNotificationType type = subject.contains("tagged")
                            ? SongNotificationType.TAGGED
                            : SongNotificationType.NEW;

                    String artist = song.artistString();
                    String title = song.name();
                    LocalDateTime publishDate = song.publishDate();
                    SongType songType = song.songType();
                    List<Tag> tags = Stream.ofNullable(song.tags()).flatMap(Collection::stream)
                            .map(Tag::of)
                            .sorted(Comparator.comparing(Tag::count)
                                    .thenComparing(Tag::name))
                            .collect(Collectors.toList());
                    List<PV> pvs = Stream.ofNullable(song.pvs()).flatMap(Collection::stream)
                            .map(PV::of)
                            .sorted(Comparator.<PV, String>comparing(pv -> pv.service().name())
                                    .thenComparing(pv -> pv.pvType().name()))
                            .collect(Collectors.toList());

                    return new SongNotification(
                            id,
                            subject,
                            body,
                            type,
                            songId,
                            songType,
                            tags,
                            pvs,
                            title,
                            artist,
                            publishDate
                    );
                });
    }

    @SuppressWarnings("ReturnCount")
    private Pair<NotificationType, Integer> detectNotificationType(UserMessageContract notification) {
        Pair<NotificationType, Integer> unknown = Pair.of(UNKNOWN, 0);
        String body = notification.body();
        if (body == null) {
            return unknown; // must never happen, because body is fetched
        }

        Pattern pattern = Pattern.compile(".*\\(https?://vocadb.net/(\\w+)/(\\d+)\\).*");
        Matcher matcher = pattern.matcher(body);
        if (!matcher.find()) {
            return unknown;
        }

        String typeStr = matcher.group(1);
        NotificationType type = NotificationType.of(typeStr);
        if (type == UNKNOWN) {
            return unknown;
        }

        String id = matcher.group(2);
        int matchedId = Integer.parseInt(id);
        return Pair.of(type, matchedId);
    }
}


