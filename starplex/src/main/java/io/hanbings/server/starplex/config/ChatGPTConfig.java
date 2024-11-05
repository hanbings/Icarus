package io.hanbings.server.starplex.config;

import lombok.Data;
import lombok.experimental.Accessors;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

@Data
@Component
@Accessors(fluent = true)
public class ChatGPTConfig {
    @Value("${chatgpt.base.api}")
    String baseAPI;

    @Value("${chatgpt.api.key}")
    String apiKey;
}
