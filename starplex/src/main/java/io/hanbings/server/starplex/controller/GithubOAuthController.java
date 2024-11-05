package io.hanbings.server.starplex.controller;

import io.hanbings.server.starplex.auth.GithubToken;
import io.hanbings.server.starplex.data.Message;
import io.hanbings.server.starplex.data.Token;
import io.hanbings.server.starplex.model.Account;
import io.hanbings.server.starplex.service.AccountService;
import io.hanbings.server.starplex.service.GithubOAuthService;
import io.hanbings.server.starplex.service.TokenService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.io.IOException;
import java.util.Map;
import java.util.Set;

@RestController
@RequiredArgsConstructor
public class GithubOAuthController {
    final TokenService tokenService;
    final AccountService accountService;
    final GithubOAuthService githubOAuthService;

    @GetMapping("/oauth/github")
    public String login(
            @RequestParam(
                    required = false,
                    defaultValue = "false",
                    name = "private_repository") boolean privateRepository) {
        return githubOAuthService.getAuthorizationUrl(privateRepository);
    }

    @GetMapping("/oauth/github/callback")
    public Object callback(@RequestParam String code, @RequestParam String state) throws IOException {
        GithubToken accessToken = githubOAuthService.getAccessToken(code, state);
        if (accessToken == null) {
            return Map.of(
                    "code", 400,
                    "message", Message.Messages.OAUTH_PROVIDER_INVALID
            );
        }

        Account account = accountService.createAccountOrLogin(accessToken.accessToken());
        if (account == null) {
            return Map.of(
                    "code", 400,
                    "message", Message.Messages.OAUTH_PROVIDER_INVALID
            );
        }

        Token token = tokenService.signature(account.openid(), 30 * 24 * 60 * 60, Set.of("AROUND"));

        return Map.of(
                "code", Message.ReturnCode.SUCCESS,
                "message", Message.Messages.SUCCESS,
                "account", account,
                "token", token
        );
    }
}
