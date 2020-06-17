package vocadb.notification.reader.client.model.pv;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.IOException;
import java.util.Map;

class RawJsonToMapDeserializer extends JsonDeserializer<Map<String, String>> {
    @Override
    public Map<String, String> deserialize(JsonParser p, DeserializationContext ctxt) throws IOException {
        ObjectMapper mapper = (ObjectMapper) p.getCodec();
        return mapper.readValue(p.getValueAsString(), new TypeReference<>() {});
    }
}
