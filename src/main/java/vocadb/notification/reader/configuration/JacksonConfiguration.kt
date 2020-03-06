package vocadb.notification.reader.configuration

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import com.fasterxml.jackson.module.afterburner.AfterburnerModule
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.codec.json.Jackson2JsonEncoder
import org.zalando.problem.ProblemModule
import org.zalando.problem.violations.ConstraintViolationProblemModule

@Configuration
class JacksonConfiguration {
    @Bean
    fun javaTimeModule(): JavaTimeModule = JavaTimeModule()

    @Bean
    fun jdk8TimeModule(): Jdk8Module = Jdk8Module()

    @Bean
    fun afterburnerModule(): AfterburnerModule = AfterburnerModule()

    @Bean
    fun problemModule(): ProblemModule = ProblemModule()

    @Bean
    fun constraintViolationProblemModule(): ConstraintViolationProblemModule = ConstraintViolationProblemModule()

    @Bean
    fun jackson2JsonEncoder(objectMapper: ObjectMapper): Jackson2JsonEncoder = Jackson2JsonEncoder(objectMapper)
}
