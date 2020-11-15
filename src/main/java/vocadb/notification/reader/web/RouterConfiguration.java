package vocadb.notification.reader.web;

import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerResponse;
import static org.springframework.http.HttpMethod.DELETE;
import static org.springframework.web.reactive.function.server.RequestPredicates.GET;
import static org.springframework.web.reactive.function.server.RequestPredicates.method;
import static org.springframework.web.reactive.function.server.RequestPredicates.path;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Configuration(proxyBeanMethods = false)
@RequiredArgsConstructor
public class RouterConfiguration {
    private final WebHandlers webHandlers;

    @Bean
    RouterFunction<ServerResponse> routerFunction() {
        return route(GET("/api/v1/account"), webHandlers::getAccount)
            .andNest(path("/api/v1/notifications"),
                route(method(HttpMethod.GET), webHandlers::getNotifications)
                    .andRoute(method(DELETE), webHandlers::deleteNotifications)
            );
    }

}
