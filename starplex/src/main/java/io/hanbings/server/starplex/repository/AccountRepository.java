package io.hanbings.server.starplex.repository;

import io.hanbings.server.starplex.model.Account;
import org.jetbrains.annotations.NotNull;
import org.springframework.data.mongodb.repository.MongoRepository;

public interface AccountRepository extends MongoRepository<Account, String> {
    @Override
    <T extends Account> @NotNull T save(@NotNull T entity);
}