package io.hanbings.server.starplex.controller;

import io.hanbings.server.starplex.data.Message;
import io.hanbings.server.starplex.model.SimpleRating;
import io.hanbings.server.starplex.service.RankService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;
import java.util.Map;

@RestController
@RequiredArgsConstructor
public class RankController {
    final RankService rankService;

    @GetMapping("/github/{username}")
    public Object getRating(@PathVariable String username) {
        SimpleRating rating = rankService.getRating(username);
        if (rating == null) {
            return Map.of(
                    "code", Message.ReturnCode.NOT_FOUND,
                    "message", Message.Messages.NOT_FOUND
            );
        }

        return Map.of(
                "code", Message.ReturnCode.SUCCESS,
                "message", Message.Messages.SUCCESS,
                "rating", rating
        );
    }

    @GetMapping("/rank")
    public Object getRank(
            @RequestParam(required = false, defaultValue = "false") boolean star,
            @RequestParam(required = false, defaultValue = "false") boolean follower,
            @RequestParam(required = false, defaultValue = "false") boolean desc
    ) {
        List<SimpleRating> ratings = rankService.getRank(star, follower, desc);

        if (ratings == null) {
            return Map.of(
                    "code", Message.ReturnCode.NOT_FOUND,
                    "message", Message.Messages.NOT_FOUND
            );
        }

        return Map.of(
                "code", Message.ReturnCode.SUCCESS,
                "message", Message.Messages.SUCCESS,
                "rank", ratings
        );
    }
}
