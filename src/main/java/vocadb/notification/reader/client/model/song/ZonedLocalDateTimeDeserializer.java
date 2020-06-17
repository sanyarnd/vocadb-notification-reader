package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import java.io.IOException;
import java.time.LocalDateTime;
import java.time.ZoneOffset;
import java.time.ZonedDateTime;
import java.time.format.DateTimeParseException;

class ZonedLocalDateTimeDeserializer extends JsonDeserializer<ZonedDateTime> {

    @Override public ZonedDateTime deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        String date = p.getValueAsString();
        try {
            return ZonedDateTime.parse(date);
        } catch (DateTimeParseException ex) {
            LocalDateTime localDate = LocalDateTime.parse(date);
            return ZonedDateTime.of(localDate, ZoneOffset.UTC);
        }
    }
}
