package vocadb.notification.reader.web;

import org.springframework.web.bind.annotation.ControllerAdvice;
import org.zalando.problem.spring.webflux.advice.ProblemHandling;
import org.zalando.problem.spring.webflux.advice.security.SecurityAdviceTrait;

@ControllerAdvice
class ExceptionHandling implements ProblemHandling, SecurityAdviceTrait {
}
