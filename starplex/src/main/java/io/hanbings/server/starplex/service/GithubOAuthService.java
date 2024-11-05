package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.config.GithubOAuthConfig;
import io.hanbings.server.starplex.utils.RandomUtils;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.HashMap;

@Service
@RequiredArgsConstructor
public class GithubOAuthService {
    final GithubOAuthConfig config;

    public String getAuthorizationUrl(boolean privateRepository) {
        HashMap<String, String> params = new HashMap<>() {{
            put("client_id", config.clientId());
            put("redirect_uri", config.callbackUrl());
            put("scope", privateRepository ? "read:user,user:email,repo" : "read:user,user:email,public_repo");
            put("state", RandomUtils.uuid());
        }};

        return "https://github.com/login/oauth/authorize?" +
                params.entrySet()
                        .stream()
                        .map(e -> e.getKey() + "=" + e.getValue())
                        .reduce((a, b) -> a + "&" + b)
                        .orElse("");
    }

    public String getAccessToken(String callback) {

        return null;
    }
}
