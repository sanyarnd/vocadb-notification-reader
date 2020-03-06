package vocadb.notification.reader.client.util

import com.fasterxml.jackson.core.JsonParser
import com.fasterxml.jackson.databind.DeserializationContext
import com.fasterxml.jackson.databind.JsonDeserializer
import java.time.LocalDateTime
import java.time.ZoneOffset
import java.time.ZonedDateTime
import java.time.format.DateTimeParseException

class CommaSeparatedStringToCollectionDeserializer : JsonDeserializer<Collection<String>>() {
    override fun deserialize(p: JsonParser, ctxt: DeserializationContext): Collection<String> {
        return p.valueAsString
            .split(',')
            .map { it.trim() }
    }
}

class ZonedLocalDateTimeDeserializer : JsonDeserializer<ZonedDateTime>() {
    override fun deserialize(p: JsonParser, ctxt: DeserializationContext): ZonedDateTime {
        val date = p.valueAsString
        return try {
            ZonedDateTime.parse(date)
        } catch (ex: DateTimeParseException) {
            val localDate = LocalDateTime.parse(date)
            ZonedDateTime.of(localDate, ZoneOffset.UTC)
        }
    }
}
