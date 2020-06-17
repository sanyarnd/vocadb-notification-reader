package vocadb.notification.reader.configuration;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.boot.autoconfigure.AutoConfigureAfter;
import org.springframework.boot.autoconfigure.web.servlet.DispatcherServletAutoConfiguration;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.annotation.Order;
import org.springframework.web.reactive.config.WebFluxConfigurer;
import org.springframework.web.server.WebExceptionHandler;
import org.zalando.problem.spring.webflux.advice.ProblemExceptionHandler;
import org.zalando.problem.spring.webflux.advice.ProblemHandling;

@Configuration
@AutoConfigureAfter(DispatcherServletAutoConfiguration.class)
class WebConfiguration implements WebFluxConfigurer {

    /**
     * In WebFlux, if a request handler is not called, then the ControllerAdvice will not be used.
     * So for ResponseStatusAdviceTrait for a 404 Not found, MethodNotAllowedAdviceTrait, NotAcceptableAdviceTrait,
     * and UnsupportedMediaTypeAdviceTrait it is required to add a specific WebExceptionHandler:
     * <p>
     * The handler must have precedence over WebFluxResponseStatusExceptionHandler and ErrorWebExceptionHandler
     */
    @Bean
    @Order(-2)
    public WebExceptionHandler problemExceptionHandler(ObjectMapper mapper, ProblemHandling problemHandling) {
        return new ProblemExceptionHandler(mapper, problemHandling);
    }
}
