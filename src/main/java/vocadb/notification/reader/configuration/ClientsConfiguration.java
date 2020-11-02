package vocadb.notification.reader.configuration;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.Scope;
import vocadb.notification.reader.client.TouhouDbClient;
import vocadb.notification.reader.client.UtaiteDbClient;
import vocadb.notification.reader.client.VocaDbClient;

@Configuration(proxyBeanMethods = false)
public class ClientsConfiguration {

    @Bean @Scope("prototype")
    VocaDbClient vocaDbClient(ObjectMapper objectMapper) {
        return new VocaDbClient.Builder()
                .setObjectMapper(objectMapper)
                .build();
    }

    @Bean @Scope("prototype")
    UtaiteDbClient taiteDbClient(ObjectMapper objectMapper) {
        return new UtaiteDbClient.Builder()
                .setObjectMapper(objectMapper)
                .build();
    }

    @Bean @Scope("prototype")
    TouhouDbClient touhouDbClient(ObjectMapper objectMapper) {
        return new TouhouDbClient.Builder()
                .setObjectMapper(objectMapper)
                .build();
    }
}
