package io.hanbings.server.starplex.service;

import io.hanbings.server.MakemakeClient;
import io.hanbings.server.starplex.data.AccountDto;
import io.hanbings.server.starplex.model.Account;
import io.hanbings.server.starplex.model.SimpleRating;
import io.hanbings.server.starplex.repository.AccountRepository;
import io.hanbings.server.starplex.repository.SimpleRatingRepository;
import io.hanbings.server.starplex.utils.SimpleRank;
import lombok.RequiredArgsConstructor;
import org.kohsuke.github.*;
import org.springframework.stereotype.Service;

import java.io.IOException;
import java.util.Collection;

@Service
@RequiredArgsConstructor
public class AccountService {
    final AccountRepository accountRepository;
    final SimpleRatingRepository simpleRatingRepository;
    final MakemakeClient makemakeClient;

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

    public SimpleRating getRating(String openId, boolean refresh) throws IOException {
        Account account = accountRepository.findByOpenid(openId);
        if (account == null) return null;

        SimpleRating rating = simpleRatingRepository.findByOpenid(openId);
        if (rating == null || refresh) {
            GitHub github = new GitHubBuilder().withOAuthToken(account.token()).build();
            GHUser user = github.getMyself();
            Collection<GHRepository> repositories = user.getRepositories().values();

            SimpleRank simpleRank = new SimpleRank(new SimpleRank.Statistics(user, repositories));

            int star = repositories.stream().mapToInt(GHRepository::getStargazersCount).sum();
            SimpleRating simpleRating = new SimpleRating(
                    openId,
                    System.currentTimeMillis(),
                    0,
                    user.getLogin(),
                    user.getName(),
                    user.getAvatarUrl(),
                    user.getCompany(),
                    user.getLocation(),
                    null,
                    user.getTwitterUsername(),
                    star,
                    user.getFollowers().size(),
                    simpleRank.rating(),
                    0,
                    null,
                    null,
                    null,
                    user.getBlog()
            );

            if (rating == null) {
                rating = simpleRatingRepository.save(simpleRating);
            } else {
                rating = simpleRatingRepository.updateSimpleRatingByOpenid(openId, simpleRating);
            }
        }

        return rating;
    }
}
