package io.hanbings.server.starplex.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import io.hanbings.server.starplex.utils.SimpleRank;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.mongodb.core.mapping.Field;

import java.util.List;

@Document("simple_rating")
public record SimpleRating(
        @Id @JsonProperty("openid") String openid,
        @Field("created") @JsonProperty("created") long created,
        @Field("rank") @JsonProperty("rank") int rank,
        @Field("username") @JsonProperty("username") @NotNull String username,
        @Field("nickname") @JsonProperty("nickname") @NotNull String nickname,
        @Field("avatar") @JsonProperty("avatar") @NotNull String avatar,
        @Field("company") @JsonProperty("company") @Nullable String company,
        @Field("location") @JsonProperty("location") @Nullable String location,
        @Field("country") @JsonProperty("country") @Nullable String country,
        @Field("twitter") @JsonProperty("twitter") @Nullable String twitter,

        @Field("star") @JsonProperty("star") int star,
        @Field("Followers") @JsonProperty("Followers") int followers,
        @Field("rating") @JsonProperty("rating") @NotNull SimpleRank.Rating rating,

        @Field("ai_rating") @JsonProperty("ai_rating") int aiRating,
        @Field("languages") @JsonProperty("languages") @Nullable List<String> languages,
        @Field("repositories") @JsonProperty("repositories") @Nullable List<String> repositories,


        @Field("summarize") @JsonProperty("summarize") @Nullable String summarize,
        @Field("blog") @JsonProperty("blog") @Nullable String blog
) {
}
