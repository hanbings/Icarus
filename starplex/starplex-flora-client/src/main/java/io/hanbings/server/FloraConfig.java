package io.hanbings.server;

import java.util.List;

public record FloraConfig(
        String name,
        String secret,
        List<String> endpoints
) {
}
