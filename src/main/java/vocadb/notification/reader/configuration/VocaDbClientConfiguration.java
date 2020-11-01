package vocadb.notification.reader.configuration;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Scope;
import vocadb.notification.reader.client.VocaDbClient;

@Configuration(proxyBeanMethods = false)
public class VocaDbClientConfiguration {

    @Bean @Scope("prototype")
    VocaDbClient vocaDbClient(ObjectMapper objectMapper) {
        return new VocaDbClient.Builder()
                .setObjectMapper(objectMapper)
                .build();
    }
}
