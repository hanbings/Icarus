package io.hanbings.server.starplex.data;

import org.jetbrains.annotations.Nullable;

public record ChatCPTCountry(
        @Nullable String country,
        @Nullable String address,
        float probability
) {
}
