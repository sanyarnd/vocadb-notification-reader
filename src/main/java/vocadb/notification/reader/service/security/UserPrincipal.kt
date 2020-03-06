package vocadb.notification.reader.service.security

import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.userdetails.User
import org.springframework.security.core.userdetails.UserDetails
import vocadb.notification.reader.client.model.UserForApiContract

class VocaDbPrincipal(
    val vocadbUser: UserForApiContract,
    val cookies: List<String>,
    authorities: Collection<GrantedAuthority>
) : UserDetails by User(
    vocadbUser.name,
    "not-stored",
    true,
    true,
    true,
    true,
    authorities
) {
    override fun toString(): String = "VocaDbPrincipal(vocaDbUser=$vocadbUser)"
}
