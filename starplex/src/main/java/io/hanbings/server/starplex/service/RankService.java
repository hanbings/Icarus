package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.model.SimpleRating;
import io.hanbings.server.starplex.repository.SimpleRatingRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
@RequiredArgsConstructor
public class RankService {
    final SimpleRatingRepository simpleRatingRepository;

    public SimpleRating getRating(String username) {
        return simpleRatingRepository.findByUsername(username);
    }

    public List<SimpleRating> getRank(boolean star, boolean follower, boolean desc) {
        Sort.Order orderFirst = star ? Sort.Order.by("star") : Sort.Order.by("follower");
        Sort.Order orderSecond = follower ? Sort.Order.by("follower") : Sort.Order.by("star");

        Pageable pageable = PageRequest.of(
                0,
                10,
                desc ? Sort.by(orderFirst, orderSecond).descending() : Sort.by(orderFirst, orderSecond).ascending()
        );

        return simpleRatingRepository.findAll(pageable).getContent();
    }
}
