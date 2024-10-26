package io.hanbings.server.starplex.controller;

import io.hanbings.server.starplex.annotation.StarplexPermissionCheck;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class AccountController {
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
    public Object account() {
        return "100";
    }

    @DeleteMapping("/account/{openid}")
    @StarplexPermissionCheck(access = {"AROUND"})
    public Object deleteAccount(@PathVariable String openid) {
        return "100";
    }
}
