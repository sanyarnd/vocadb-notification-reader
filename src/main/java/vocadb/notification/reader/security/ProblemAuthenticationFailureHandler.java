package vocadb.notification.reader.security;

import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.AuthenticationException;
import org.springframework.security.web.server.WebFilterExchange;
import org.springframework.security.web.server.authentication.ServerAuthenticationFailureHandler;
import org.springframework.stereotype.Component;
import org.zalando.problem.Problem;
import reactor.core.publisher.Mono;
import vocadb.notification.reader.advice.ProblemAdvice;

@Component
@Slf4j
@RequiredArgsConstructor
class ProblemAuthenticationFailureHandler implements ServerAuthenticationFailureHandler {
    private final ProblemAdvice advice;

    @Override
    public Mono<Void> onAuthenticationFailure(WebFilterExchange webFilterExchange, AuthenticationException exception) {
        Mono<ResponseEntity<Problem>> response = advice.create(exception, webFilterExchange.getExchange());
        return response.and(Mono.empty());
    }
}
