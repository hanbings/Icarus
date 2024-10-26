package io.hanbings.server.starplex.data;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.Set;

public record Token(
        @JsonProperty("token") String token,
        @JsonProperty("belong") String belong,
        @JsonProperty("permissions") Set<String> permissions,
        @JsonProperty("expire") long expire,
        @JsonProperty("created") long created
) {
}
