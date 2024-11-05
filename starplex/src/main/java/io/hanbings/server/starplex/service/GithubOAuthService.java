package io.hanbings.server.starplex.service;

import com.google.gson.Gson;
import io.hanbings.server.starplex.auth.GithubToken;
import io.hanbings.server.starplex.config.GithubOAuthConfig;
import io.hanbings.server.starplex.utils.RandomUtils;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import okhttp3.FormBody;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Service;

import java.util.HashMap;
import java.util.Map;

@Slf4j
@Service
@RequiredArgsConstructor
public class GithubOAuthService {
    final GithubOAuthConfig config;
    final Map<String, Long> stateMap = new HashMap<>();

    static Gson gson = new Gson();
    static OkHttpClient okHttpClient = new OkHttpClient();

    public String getAuthorizationUrl(boolean privateRepository) {
        String state = RandomUtils.uuid();
        HashMap<String, String> params = new HashMap<>() {{
            put("client_id", config.clientId());
            put("redirect_uri", config.callbackUrl());
            put("scope", privateRepository ? "read:user,user:email,repo" : "read:user,user:email,public_repo");
            put("state", state);
        }};

        stateMap.put(state, System.currentTimeMillis());

        return "https://github.com/login/oauth/authorize?" +
                params.entrySet()
                        .stream()
                        .map(e -> e.getKey() + "=" + e.getValue())
                        .reduce((a, b) -> a + "&" + b)
                        .orElse("");
    }

    @Scheduled(initialDelay=1000, fixedDelay=30000)
    public void checkState() {
        stateMap.forEach((key, value) -> {
            if (value + 30000 < System.currentTimeMillis()) {
                stateMap.remove(key);
            }
        });
    }

    public GithubToken getAccessToken(String code, String state) {
        if (stateMap.get(state) == null) return null;

        FormBody body = new FormBody.Builder()
                .add("client_id", config.clientId())
                .add("client_secret", config.clientSecret())
                .add("code", code)
                .add("state", state)
                .build();

        Request request = new Request.Builder()
                .url("https://github.com/login/oauth/access_token")
                .addHeader("Accept", "application/json")
                .post(body)
                .build();

        try (var response = okHttpClient.newCall(request).execute()) {
            if (response.body() == null) return null;
            String result = response.body().string();

            return gson.fromJson(result, GithubToken.class);
        } catch (Exception e) {
            log.error(e.getMessage(), e);
        }

        return null;
    }
}
