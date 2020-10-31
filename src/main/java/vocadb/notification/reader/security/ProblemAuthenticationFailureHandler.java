package vocadb.notification.reader.security;

import edu.umd.cs.findbugs.annotations.SuppressFBWarnings;
import java.util.Objects;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.core.ResolvableType;
import org.springframework.core.codec.Hints;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.codec.json.Jackson2JsonEncoder;
import org.springframework.http.server.reactive.ServerHttpResponse;
import org.springframework.security.authentication.AccountStatusException;
import org.springframework.security.authentication.AuthenticationServiceException;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.core.AuthenticationException;
import org.springframework.security.web.server.WebFilterExchange;
import org.springframework.security.web.server.authentication.ServerAuthenticationFailureHandler;
import org.springframework.stereotype.Component;
import org.springframework.web.server.ServerWebExchange;
import org.zalando.problem.Problem;
import org.zalando.problem.Status;
import reactor.core.publisher.Mono;

@Component
@Slf4j
@RequiredArgsConstructor
class ProblemAuthenticationFailureHandler implements ServerAuthenticationFailureHandler {
    private final Jackson2JsonEncoder encoder;

    @Override
    public Mono<Void> onAuthenticationFailure(WebFilterExchange webFilterExchange, AuthenticationException exception) {
        ServerHttpResponse response = webFilterExchange.getExchange().getResponse();
        if (response.isCommitted()) {
            log.error("Response is already committed. Unable to write exception "
                    + "${exception.javaClass.simpleName}: ${exception.localizedMessage}");
            return Mono.empty();
        }

        Problem problem = exceptionMapper(exception);
        int status = Objects.requireNonNull(problem.getStatus()).getStatusCode();

        response.setStatusCode(HttpStatus.valueOf(status));
        return toJson(problem, webFilterExchange.getExchange());
    }

    private Mono<Void> toJson(Object body, ServerWebExchange exchange) {
        return exchange.getResponse().writeWith(
                encoder.encode(
                        Mono.just(body),
                        exchange.getResponse().bufferFactory(),
                        ResolvableType.forInstance(body),
                        MediaType.APPLICATION_JSON,
                        Hints.from(Hints.LOG_PREFIX_HINT, exchange.getLogPrefix())
                )
        );
    }

    @SuppressFBWarnings("NP_NULL_PARAM_DEREF")
    private Problem exceptionMapper(AuthenticationException exception) {
        Status status = Status.UNAUTHORIZED;
        String message = null;

        if (exception instanceof AuthenticationServiceException) {
            status = Status.INTERNAL_SERVER_ERROR;
            message = "Internal authentication error";
        } else if (exception instanceof AccountStatusException) {
            status = Status.FORBIDDEN;
            message = "User is either disabled or has expired credentials";
        } else if (exception instanceof BadCredentialsException) {
            message = "Invalid credentials";
        }

        return Problem.valueOf(status, message);
    }
}
