package vocadb.notification.reader.client;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.ProxySelector;
import java.net.URI;
import java.net.http.HttpClient.Redirect;
import java.time.Duration;
import lombok.Getter;

@Getter
public final class TouhouDbClient extends BaseClient {
    private TouhouDbClient(
            ObjectMapper objectMapper,
            URI baseUrl,
            ProxySelector proxySelector,
            Duration connectTimeout,
            Redirect followRedirects
    ) {
        super(objectMapper, baseUrl, proxySelector, connectTimeout, followRedirects);
    }

    public static class Builder extends BaseBuilder<TouhouDbClient> {
        public TouhouDbClient build() {
            if (baseUrl == null) {
                baseUrl = URI.create("https://vocadb.net");
            }
            if (objectMapper == null) {
                objectMapper = defaultObjectMapper();
            }

            return new TouhouDbClient(objectMapper, baseUrl, proxySelector, connectTimeout, followRedirects);
        }


    }
}
