package io.hanbings.server.starplex.controller;

import io.hanbings.server.starplex.annotation.StarplexPermissionCheck;
import io.hanbings.server.starplex.data.AccountDto;
import io.hanbings.server.starplex.data.Message;
import io.hanbings.server.starplex.security.Header;
import io.hanbings.server.starplex.service.AccountService;
import jakarta.servlet.http.HttpServletRequest;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequiredArgsConstructor
public class AccountController {
    final AccountService accountService;

    @GetMapping("/rating")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object getRating() {
        return "100";
    }

    @GetMapping("/rating/refresh")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object refreshRating() {
        return "100";
    }

    @GetMapping("/rating/simple")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object getSimpleRating() {
        return "100";
    }

    @GetMapping("/rating/simple/refresh")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object refresh() {
        return "100";
    }

    @GetMapping("/account")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object account(HttpServletRequest request) {
        String openId = request.getHeader(Header.ACCOUNT);
        if (openId == null) {
            return Map.of(
                    "code", Message.ReturnCode.UNAUTHORIZED,
                    "message", Message.Messages.UNAUTHORIZED
            );
        }

        AccountDto account = accountService.getAccount(openId);
        if (account == null) {
            return Map.of(
                    "code", Message.ReturnCode.UNAUTHORIZED,
                    "message", Message.Messages.UNAUTHORIZED
            );
        }

        return Map.of(
                "code", Message.ReturnCode.SUCCESS,
                "message", Message.Messages.SUCCESS,
                "account", account
        );
    }

    @DeleteMapping("/account/{openid}")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object deleteAccount(HttpServletRequest request, @PathVariable String openid) {
        String openId = request.getHeader(Header.ACCOUNT);
        if (openId == null || !openId.equals(openid)) {
            return Map.of(
                    "code", Message.ReturnCode.UNAUTHORIZED,
                    "message", Message.Messages.UNAUTHORIZED
            );
        }

        accountService.deleteAccount(openid);

        return Map.of(
                "code", Message.ReturnCode.SUCCESS,
                "message", Message.Messages.SUCCESS
        );
    }
}
