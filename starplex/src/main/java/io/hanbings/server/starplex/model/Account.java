package io.hanbings.server.starplex.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jetbrains.annotations.NotNull;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.mongodb.core.mapping.Field;

@Document("account")
public record Account(
        @Id @JsonProperty("openid") @NotNull String openid,
        @Field("created") @JsonProperty("created") long created,
        @Field("nickname") @JsonProperty("nickname") String nickname,
        @Field("avatar") @JsonProperty("avatar") String avatar,
        @Field("email") @JsonProperty("email") String email,
        @Field("token") @JsonProperty("token") String token
) { }
