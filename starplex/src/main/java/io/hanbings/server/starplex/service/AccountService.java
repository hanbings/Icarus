package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.model.Account;
import io.hanbings.server.starplex.utils.SimpleRank;
import org.springframework.stereotype.Service;

@Service
public class AccountService {
    public Account createAccountOrLogin(String openid, String token) {
        return null;
    }

    public SimpleRank.Rating getRating(String token) {
        return null;
    }
}
