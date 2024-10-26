package io.hanbings.server.starplex.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class GithubOAuthController {
    @GetMapping("/oauth/github")
    public void login() {

    }

    @PostMapping("/oauth/github/callback")
    public void callback() {

    }
}
