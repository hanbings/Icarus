package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.data.Token;
import io.hanbings.server.starplex.utils.RandomUtils;
import io.hanbings.server.starplex.utils.TimeUtils;
import lombok.RequiredArgsConstructor;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

import java.util.Set;

@Service
@RequiredArgsConstructor
public class TokenService {
    // static Map<String, Token> tokens = new ConcurrentHashMap<>();
    final RedisTemplate<String, Token> redisTemplate;

    public Token signature(String belong, long expire, Set<String> access) {
        Token token = new Token(
                RandomUtils.uuid(),
                belong,
                access,
                TimeUtils.getMilliUnixTime(),
                TimeUtils.getMilliUnixTime() + expire
        );

        redisTemplate.opsForValue().set(token.token(), token);

        return token;
    }

    public void register(String token, String belong, long expire, Set<String> access) {
        Token t = new Token(
                token,
                belong,
                access,
                TimeUtils.getMilliUnixTime(),
                TimeUtils.getMilliUnixTime() + expire
        );

        redisTemplate.opsForValue().set(token, t);
    }

    public Token parse(String header) {
        String authorization = header.substring(7);

        return redisTemplate.opsForValue().get(authorization);
    }

    public Token get(String token) {
        return redisTemplate.opsForValue().get(token);
    }

    public void revoke(String token) {
        redisTemplate.delete(token);
    }

    public boolean checkAccess(String token, Set<String> access) {
        Token t = redisTemplate.opsForValue().get(token);
        if (t == null) return false;

        return t.permissions().containsAll(access);
    }

    public boolean checkAccess(Token token, String permission) {
        return token.permissions().contains(permission);
    }

    public boolean checkAccess(Token token, Set<String> permissions) {
        return token.permissions().containsAll(permissions);
    }

    public static class Expire {
        public static final long MINUTE = 60 * 1000;
        public static final long HOUR = 60 * MINUTE;
        public static final long DAY = 24 * HOUR;
        public static final long WEEK = 7 * DAY;
        public static final long MONTH = 30 * DAY;
    }
}
