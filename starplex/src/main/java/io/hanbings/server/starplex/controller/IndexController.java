package io.hanbings.server.starplex.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
public class IndexController {
    static final Map<String, String> INDEX = Map.of(
            "name", "Starplex",
            "version", "1.0.0",
            "author", "hanbings",
            "github", "https://github.com/hanbings/icarus",
            "api", "https://api.beta.icaruspw.dev"
    );

    @GetMapping("/")
    public Object index() {
        return INDEX;
    }

    @GetMapping("/api/v0")
    public Object api() {
        return INDEX;
    }
}
