package vocadb.notification.reader.configuration

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.context.annotation.Scope
import vocadb.notification.reader.client.VocaDbClient

@Configuration
class VocaDbClientConfiguration {

    @Bean @Scope("prototype")
    fun vocaDbClient(
        objectMapper: ObjectMapper
    ): VocaDbClient = VocaDbClient(objectMapper)

}
