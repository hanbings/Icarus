package io.hanbings.server.starplex.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.mongodb.core.mapping.Field;

import java.util.List;

@Document("account")
public record Account(
        @Id @JsonProperty("openid") @NotNull String openid,
        @Field("created") @JsonProperty("created") long created,
        @Field("username") @JsonProperty("username") @NotNull String username,
        @Field("avatar") @JsonProperty("avatar") @NotNull String avatar,
        @Field("token") @JsonProperty("token") @NotNull String token,
        @Field("email") @JsonProperty("email") @Nullable List<String> email,
        @Field("nickname") @JsonProperty("nickname") @NotNull String nickname
) {
}
