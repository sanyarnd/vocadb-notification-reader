package vocadb.notification.reader.client;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.ProxySelector;
import java.net.URI;
import java.net.http.HttpClient.Redirect;
import java.time.Duration;
import lombok.Getter;

@Getter
public final class UtaiteDbClient extends BaseClient {
    private UtaiteDbClient(
            ObjectMapper objectMapper,
            URI baseUrl,
            ProxySelector proxySelector,
            Duration connectTimeout,
            Redirect followRedirects
    ) {
        super(objectMapper, baseUrl, proxySelector, connectTimeout, followRedirects);
    }

    public static class Builder extends BaseBuilder<UtaiteDbClient> {
        public UtaiteDbClient build() {
            if (baseUrl == null) {
                baseUrl = URI.create("https://utaitedb.net/");
            }
            if (objectMapper == null) {
                objectMapper = defaultObjectMapper();
            }

            return new UtaiteDbClient(objectMapper, baseUrl, proxySelector, connectTimeout, followRedirects);
        }
    }
}
