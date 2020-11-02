package vocadb.notification.reader.client;

import java.net.URI;
import java.net.URLEncoder;
import java.net.http.HttpRequest;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import lombok.experimental.UtilityClass;

@UtilityClass
public class HttpUtils {

    public static final String APPLICATION_JSON = "application/json";
    public static final String ACCEPT = "Accept";

    public static HttpRequest getRequest(URI uri) {
        return HttpRequest.newBuilder().header(ACCEPT, APPLICATION_JSON).uri(uri).GET().build();
    }

    public static HttpRequest deleteRequest(URI uri) {
        return HttpRequest.newBuilder().header(ACCEPT, APPLICATION_JSON).uri(uri).DELETE().build();
    }

    public static HttpRequest.BodyPublisher ofFormData(Map<Object, Object> data) {
        var builder = new StringBuilder();
        for (Map.Entry<Object, Object> entry : data.entrySet()) {
            if (builder.length() > 0) {
                builder.append('&');
            }

            builder.append(URLEncoder.encode(entry.getKey().toString(), StandardCharsets.UTF_8));
            builder.append('=');
            builder.append(URLEncoder.encode(entry.getValue().toString(), StandardCharsets.UTF_8));
        }
        return HttpRequest.BodyPublishers.ofString(builder.toString());
    }
}
