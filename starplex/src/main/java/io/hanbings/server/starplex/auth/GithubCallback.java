package io.hanbings.server.starplex.auth;

public record GithubCallback(String code, String state) {
}
