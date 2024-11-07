package io.hanbings.server.starplex.repository;

import io.hanbings.server.starplex.model.Account;
import org.jetbrains.annotations.NotNull;
import org.springframework.data.mongodb.repository.MongoRepository;
import org.springframework.data.mongodb.repository.Query;

public interface AccountRepository extends MongoRepository<Account, String> {
    @Override
    <T extends Account> @NotNull T save(@NotNull T entity);

    Account findByOpenid(String openid);

    @Query("{'openid': ?0}")
    Account updateByOpenid(String openid, Account account);

    @Query("{'openid': ?0}")
    Account updateTokenByOpenid(String openid, String token);

    void deleteByOpenid(String openid);
}
