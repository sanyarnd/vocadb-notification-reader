package vocadb.notification.reader.service.security

import kotlinx.coroutines.future.await
import kotlinx.coroutines.reactor.mono
import org.springframework.beans.factory.ObjectProvider
import org.springframework.security.authentication.AuthenticationServiceException
import org.springframework.security.authentication.BadCredentialsException
import org.springframework.security.authentication.ReactiveAuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.Authentication
import org.springframework.security.core.authority.SimpleGrantedAuthority
import org.springframework.stereotype.Component
import reactor.core.publisher.Mono
import vocadb.notification.reader.client.VocaDbClient

@Component
class VocaDbAuthenticationProvider(
    val clientProvider: ObjectProvider<VocaDbClient>
) : ReactiveAuthenticationManager {

    override fun authenticate(authentication: Authentication): Mono<Authentication> = mono {
        val username = authentication.name
        val credentials = authentication.credentials as String

        val client = clientProvider.getObject()
        val authenticated = try {
            client.authenticate(username, credentials).await()
        } catch (ex: Exception) {
            throw AuthenticationServiceException("Unable to authenticate $username", ex)
        }
        if (!authenticated) throw BadCredentialsException("Bad credentials")

        val vocadbUser = try {
            client.userApi.getCurrentUser().await()
        } catch (ex: Exception) {
            throw AuthenticationServiceException("Unable to get '$username' information", ex)
        }

        val userAuthorities = listOf(SimpleGrantedAuthority(UserAuthority.ROLE_USER))
        val p = VocaDbPrincipal(vocadbUser, ArrayList(client.cookies), userAuthorities)

        UsernamePasswordAuthenticationToken(p, p.password, p.authorities)
    }
}

