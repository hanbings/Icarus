package io.hanbings.server.starplex.config;

import lombok.Data;
import lombok.experimental.Accessors;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

@Data
@Component
@Accessors(fluent = true)
public class GithubOAuthConfig {
    @Value("${github.client.id}")
    String clientId;

    @Value("${github.client.secret}")
    String clientSecret;

    @Value("${github.callback.url}")
    String callbackUrl;
}
