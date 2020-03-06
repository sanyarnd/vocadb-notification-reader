package vocadb.notification.reader.configuration

import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.boot.autoconfigure.AutoConfigureAfter
import org.springframework.boot.autoconfigure.web.servlet.DispatcherServletAutoConfiguration
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.core.annotation.Order
import org.springframework.web.reactive.config.ResourceHandlerRegistry
import org.springframework.web.reactive.config.WebFluxConfigurer
import org.zalando.problem.spring.webflux.advice.ProblemExceptionHandler
import org.zalando.problem.spring.webflux.advice.ProblemHandling

@Configuration
@AutoConfigureAfter(DispatcherServletAutoConfiguration::class)
class WebConfiguration : WebFluxConfigurer {

    /**
     * The list of resources available for static resolution
     *
     * Default resource resolution will not allow proper JSON-response handling,
     * because it doesn't allow multiple handlers (WebMvcProperties.staticPattern is a string, not an array)
     *
     * Disable it (spring.resources.add-mappings=false) and configure separately with multiple patterns
     */
    override fun addResourceHandlers(registry: ResourceHandlerRegistry) {
        registry.addResourceHandler("/*.html", "/*.ico").addResourceLocations("classpath:/static/")

        registry.addResourceHandler("/css/**").addResourceLocations("classpath:/static/css/")
        registry.addResourceHandler("/img/**").addResourceLocations("classpath:/static/img/")
        registry.addResourceHandler("/js/**").addResourceLocations("classpath:/static/js/")
    }

    /**
     * Bypass default filters [org.springframework.web.reactive.handler.WebFluxResponseStatusExceptionHandler] and
     * [org.springframework.boot.web.reactive.error.ErrorWebExceptionHandler] (can be excluded from auto-configuration)
     * so any exceptions, like "controller not found", are handled by [vocadb.notification.reader.web.ProblemAdvice]
     */
    @Bean @Order(-2)
    fun problemExceptionHandler(mapper: ObjectMapper, problemHandling: ProblemHandling): ProblemExceptionHandler {
        return ProblemExceptionHandler(mapper, problemHandling)
    }
}
