package io.hanbings.server.starplex.service;

import io.hanbings.server.starplex.utils.SimpleRank;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Pageable;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;

@Service
public class RankService {
    public SimpleRank getRank(String username) {
        return null;
    }

    public SimpleRank getRating(String username) {
        Pageable pageable = PageRequest.of(
                1,
                10,
                Sort.by(Sort.Direction.DESC, "created")
        );

        return null;
    }
}
