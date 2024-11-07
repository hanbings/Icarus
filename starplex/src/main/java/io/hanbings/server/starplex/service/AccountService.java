package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.data.AccountDto;
import io.hanbings.server.starplex.model.Account;
import io.hanbings.server.starplex.repository.AccountRepository;
import io.hanbings.server.starplex.utils.SimpleRank;
import lombok.RequiredArgsConstructor;
import org.kohsuke.github.GHMyself;
import org.kohsuke.github.GitHub;
import org.kohsuke.github.GitHubBuilder;
import org.springframework.stereotype.Service;

import java.io.IOException;

@Service
@RequiredArgsConstructor
public class AccountService {
    final AccountRepository accountRepository;

    @SuppressWarnings("deprecation")
    public AccountDto createAccountOrLogin(String token) throws IOException {
        GitHub github = new GitHubBuilder().withOAuthToken(token).build();
        GHMyself me = github.getMyself();
        String openId = String.valueOf(me.getId());

        Account account = accountRepository.findByOpenid(openId);
        if (account == null) {
            Account newAccount = new Account(
                    openId,
                    System.currentTimeMillis(),
                    me.getLogin(),
                    me.getAvatarUrl(),
                    token,
                    me.getEmails(),
                    me.getName()
            );

            accountRepository.save(newAccount);

            return new AccountDto(
                    newAccount.openid(),
                    newAccount.created(),
                    newAccount.username(),
                    newAccount.avatar(),
                    newAccount.email(),
                    newAccount.nickname()
            );
        }

        return new AccountDto(
                account.openid(),
                account.created(),
                account.username(),
                account.avatar(),
                account.email(),
                account.nickname()
        );
    }

    public AccountDto getAccount(String openId) {
        Account account = accountRepository.findByOpenid(openId);
        if (account == null) return null;

        return new AccountDto(
                account.openid(),
                account.created(),
                account.username(),
                account.avatar(),
                account.email(),
                account.nickname()
        );
    }

    public void deleteAccount(String openId) {
        accountRepository.deleteByOpenid(openId);
    }

    public SimpleRank.Rating getRating(String token) {
        return null;
    }
}
