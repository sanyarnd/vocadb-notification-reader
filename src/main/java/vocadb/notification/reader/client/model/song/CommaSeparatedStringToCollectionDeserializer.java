package vocadb.notification.reader.client.model.song;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import java.io.IOException;
import java.util.Collection;
import java.util.stream.Collectors;
import java.util.stream.Stream;

;

class CommaSeparatedStringToCollectionDeserializer extends JsonDeserializer<Collection<String>> {
    @Override
    public Collection<String> deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        return Stream.of(p.getValueAsString().split(",")).map(String::trim).collect(Collectors.toList());
    }
}
