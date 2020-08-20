package vocadb.notification.reader.client;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.io.InputStream;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.Collection;
import java.util.concurrent.CompletionException;
import java.util.concurrent.CompletionStage;
import java.util.stream.Collectors;
import lombok.AccessLevel;
import lombok.RequiredArgsConstructor;
import org.apache.commons.lang3.tuple.Pair;

@RequiredArgsConstructor(access = AccessLevel.PROTECTED)
abstract class Api {
    protected final HttpClient client;
    protected final ObjectMapper objectMapper;
    protected final URI baseUrl;

    protected CompletionStage<Void> sendAsync(HttpRequest request) {
        return client.sendAsync(request, HttpResponse.BodyHandlers.discarding())
                .thenApplyAsync(HttpResponse::body);
    }

    protected <T> CompletionStage<T> sendAsync(HttpRequest request, TypeReference<T> typeReference) {
        return client.sendAsync(request, HttpResponse.BodyHandlers.ofInputStream())
                .thenApplyAsync(HttpResponse<InputStream>::body)
                .thenApplyAsync(body -> {
                    try {
                        return objectMapper.readValue(body, typeReference);
                    } catch (IOException e) {
                        throw new CompletionException(e);
                    }
                });
    }

    protected String toQuery(Collection<Pair<String, ?>> query) {
        return query.stream()
                .filter(e -> {
                    Object value = e.getRight();
                    // skip if no value present or parameter array is empty
                    return value != null && !(value instanceof Collection<?> && ((Collection<?>) value).isEmpty());
                })
                .map(e -> {
                    String key = e.getLeft();
                    Object value = e.getRight();

                    // special case for collections:
                    // if value is a collection, then convert every element to string and join with ','
                    // otherwise simply convert the value
                    // jackson is required for proper enum handling
                    String stringValue;
                    if (value instanceof Collection<?>) {
                        stringValue = ((Collection<?>) value).stream()
                                .map(e1 -> objectMapper.convertValue(e1, String.class))
                                .collect(Collectors.joining(","));
                    } else {
                        stringValue = objectMapper.convertValue(value, String.class);
                    }
                    return String.format("%s=%s", key, stringValue);

                })
                .collect(Collectors.joining("&"));
    }

}
