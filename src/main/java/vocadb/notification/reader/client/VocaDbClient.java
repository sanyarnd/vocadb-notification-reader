package vocadb.notification.reader.client;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.databind.json.JsonMapper;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import com.fasterxml.jackson.module.paramnames.ParameterNamesModule;
import java.io.IOException;
import java.net.CookieManager;
import java.net.HttpCookie;
import java.net.ProxySelector;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpClient.Redirect;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse.BodyHandlers;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletionStage;
import lombok.Getter;
import org.checkerframework.checker.nullness.qual.Nullable;

@Getter
public class VocaDbClient {
    private static final String SET_COOKIE = "Set-Cookie";

    private final ObjectMapper objectMapper;
    private final URI baseUrl;

    private final HttpClient client;
    private final CookieManager cookieManager;
    private final List<String> cookies;
    private final SongApi songApi;
    private final UserApi userApi;

    public VocaDbClient(
            ObjectMapper objectMapper,
            URI baseUrl,
            ProxySelector proxySelector,
            Duration connectTimeout,
            Redirect followRedirects
    ) {
        this.objectMapper = objectMapper;
        this.baseUrl = baseUrl;

        cookies = new ArrayList<>();
        cookieManager = new CookieManager();
        client = HttpClient.newBuilder()
                .connectTimeout(connectTimeout)
                .followRedirects(followRedirects)
                .cookieHandler(cookieManager)
                .proxy(proxySelector)
                .version(HttpClient.Version.HTTP_2)
                .build();

        songApi = new SongApi(this.client, this.baseUrl, this.objectMapper);
        userApi = new UserApi(this.client, this.baseUrl, this.objectMapper);
    }

    public void setCookies(List<String> cookies) {
        this.cookies.clear();
        this.cookies.addAll(cookies);

        try {
            cookieManager.put(baseUrl, Map.of(SET_COOKIE, cookies));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public CompletionStage<Boolean> authenticate(String user, String password) {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(baseUrl.resolve("/User/Login"))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .POST(HttpUtils.ofFormData(Map.of("UserName", user, "Password", password)))
                .build();

        return client.sendAsync(request, BodyHandlers.discarding())
                .thenAccept(response -> setCookies(response.headers().allValues(SET_COOKIE)))
                .thenApply(v -> isAuthenticated());
    }

    public boolean isAuthenticated() {
        List<HttpCookie> rawCookies = cookieManager.getCookieStore().get(baseUrl);
        return !rawCookies.isEmpty() && rawCookies.stream().noneMatch(HttpCookie::hasExpired);
    }

    public static class Builder {
        private @Nullable ObjectMapper objectMapper;
        private URI baseUrl = URI.create("https://vocadb.net");

        private ProxySelector proxySelector = HttpClient.Builder.NO_PROXY;
        @SuppressWarnings("MagicNumber")
        private Duration connectTimeout = Duration.ofSeconds(30);
        private Redirect followRedirects = Redirect.NEVER;

        public Builder setObjectMapper(ObjectMapper objectMapper) {
            this.objectMapper = objectMapper;
            return this;
        }

        public Builder setBaseUrl(URI baseUrl) {
            this.baseUrl = baseUrl;
            return this;
        }

        public Builder setProxySelector(ProxySelector proxySelector) {
            this.proxySelector = proxySelector;
            return this;
        }

        public Builder setConnectTimeout(Duration connectTimeout) {
            this.connectTimeout = connectTimeout;
            return this;
        }

        public Builder setFollowRedirects(Redirect followRedirects) {
            this.followRedirects = followRedirects;
            return this;
        }

        public VocaDbClient build() {
            if (objectMapper == null) {
                objectMapper = JsonMapper.builder()
                        .addModule(new Jdk8Module())
                        .addModule(new ParameterNamesModule())
                        .addModule(new JavaTimeModule())
                        .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)
                        .build();
            }

            return new VocaDbClient(objectMapper, baseUrl, proxySelector, connectTimeout, followRedirects);
        }
    }
}
