package vocadb.notification.reader.web.converter;

import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.core.convert.converter.Converter;
import org.springframework.core.convert.converter.ConverterFactory;
import org.springframework.stereotype.Component;
import org.springframework.web.bind.annotation.RequestParam;

/**
 * Convert {@link RequestParam} enums with Jackson with fallback strategy valueOf
 */
@Component
@RequiredArgsConstructor
@SuppressWarnings({"rawtypes", "unchecked"})
public class StringToEnumConverterFactory implements ConverterFactory<String, Enum> {
    private final ObjectMapper objectMapper;

    @Override
    public <T extends Enum> @NotNull Converter<String, T> getConverter(@NotNull Class<T> targetType) {
        return new StringToEnum<>(targetType, objectMapper);
    }

    @RequiredArgsConstructor
    private static class StringToEnum<T extends Enum> implements Converter<String, T> {
        private final Class<T> enumType;
        private final ObjectMapper objectMapper;

        @SuppressWarnings("ReturnCount")
        public T convert(String source) {
            if (source.isEmpty()) {
                return null;
            }
            try {
                return objectMapper.convertValue(source.trim(), enumType);
            } catch (IllegalArgumentException ex) {
                return (T) Enum.valueOf(enumType, source.trim());
            }
        }
    }
}
