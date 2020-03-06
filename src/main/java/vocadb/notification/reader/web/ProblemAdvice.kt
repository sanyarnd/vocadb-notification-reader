package vocadb.notification.reader.web

import org.springframework.web.bind.annotation.ControllerAdvice
import org.zalando.problem.spring.webflux.advice.ProblemHandling
import org.zalando.problem.spring.webflux.advice.security.SecurityAdviceTrait
import org.zalando.problem.violations.Violation
import java.net.URI
import javax.validation.ConstraintViolation

/**
 * Wrap service-level exceptions to user-friendly Problems
 */
@ControllerAdvice
class ProblemAdvice : ProblemHandling, SecurityAdviceTrait {
    override fun defaultConstraintViolationType(): URI? = null

    override fun createViolation(violation: ConstraintViolation<*>): Violation =
        Violation(formatFieldName(violation.propertyPath.last().name), violation.message)
}
