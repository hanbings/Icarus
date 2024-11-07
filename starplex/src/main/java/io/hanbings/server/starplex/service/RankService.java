package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.model.SimpleRating;
import io.hanbings.server.starplex.repository.SimpleRatingRepository;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class RankService {
    SimpleRatingRepository simpleRatingRepository;

    public SimpleRating getRating(String username) {
        return simpleRatingRepository.findByUsername(username);
    }

    public List<SimpleRating> getRank() {
        Pageable pageable = PageRequest.of(
                1,
                10,
                Sort.by(Sort.Direction.DESC, "created")
        );

        return simpleRatingRepository.findAll(pageable).getContent();
    }
}
