package io.hanbings.server.starplex.data;

import java.util.List;

public record AccountDto(
        String openid,
        long created,
        String username,
        String avatar,
        List<String> email,
        String nickname
) {
}
