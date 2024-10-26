package io.hanbings.server.starplex.auth;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.google.gson.annotations.SerializedName;

public record GithubToken(
        @SerializedName("access_token")
        @JsonProperty("access_token")
        String accessToken,
        @SerializedName("token_type")
        @JsonProperty("token_type")
        String tokenType,
        @SerializedName("scope")
        @JsonProperty("scope")
        String scope
) {
}
