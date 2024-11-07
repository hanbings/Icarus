package io.hanbings.server.starplex.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import io.hanbings.server.starplex.utils.SimpleRank;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.springframework.data.annotation.Id;
import org.springframework.data.mongodb.core.mapping.Document;
import org.springframework.data.mongodb.core.mapping.Field;

@Document("simple_rating")
public record SimpleRating(
        @Id @JsonProperty("openid") String openid,
        @Field("created") @JsonProperty("created") long created,
        @Field("rank") @JsonProperty("rank") int rank,
        @Field("username") @JsonProperty("username") @NotNull String username,
        @Field("nickname") @JsonProperty("nickname") @NotNull String nickname,
        @Field("rating") @JsonProperty("rating") @NotNull SimpleRank.Rating rating,
        @Field("summarize") @JsonProperty("summarize") @NotNull String summarize,
        @Field("rss") @JsonProperty("rss") @Nullable String rss,
        @Field("blog") @JsonProperty("blog") @Nullable String blog,
        @Field("company") @JsonProperty("company") @Nullable String company,
        @Field("location") @JsonProperty("location") @Nullable String location,
        @Field("country") @JsonProperty("country") @Nullable String country,
        @Field("twitter") @JsonProperty("twitter") @Nullable String twitter,
        @Field("star") @JsonProperty("star") int star,
        @Field("Followers") @JsonProperty("Followers") int followers
) {
}
