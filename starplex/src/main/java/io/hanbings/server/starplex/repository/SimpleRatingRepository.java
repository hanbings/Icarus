package io.hanbings.server.starplex.repository;

import io.hanbings.server.starplex.model.SimpleRating;
import org.jetbrains.annotations.NotNull;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.data.mongodb.repository.Query;

public interface SimpleRatingRepository extends MongoRepository<SimpleRating, String> {
    @Override
    <T extends SimpleRating> @NotNull T save(@NotNull T entity);

    SimpleRating findByOpenid(String openid);

    @Query("{'openid': ?0}")
    SimpleRating updateSimpleRatingByOpenid(String openid, SimpleRating simpleRating);

    @NotNull Page<SimpleRating> findAll(@NotNull Pageable pageable);

    SimpleRating findByUsername(String username);
}
