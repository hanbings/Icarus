package io.hanbings.server.starplex.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class RankController {
    @GetMapping("/github/{username}")
    public Object getRating(@PathVariable String username) {
        return "100";
    }

    @GetMapping("/rank")
    public Object getRank(@RequestParam String type) {
        return "100";
    }
}
