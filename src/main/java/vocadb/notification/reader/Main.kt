package vocadb.notification.reader

import org.springframework.boot.SpringApplication
import org.springframework.boot.autoconfigure.SpringBootApplication

@SpringBootApplication(proxyBeanMethods = false)
class Application

fun main(args: Array<String>) {
    val app = SpringApplication(Application::class.java)
    val ctx = app.run(*args)
}
