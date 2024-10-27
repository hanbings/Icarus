package io.hanbings.server.starplex.utils;

import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;
import org.kohsuke.github.GHRepository;
import org.kohsuke.github.GHUser;

import java.io.IOException;
import java.util.Collection;
import java.util.Objects;

/***
 * The algorithm is based on a simple mathematical method to perform weighted calculations on basic data.
 * <p>
 * The original algorithm is open source on Github:
 * <a href="https://github.com/AykutSarac/github-rater/blob/main/src/algorithms/index.ts">AykutSarac/github-rater</a>
 * <p>
 * Algorithm is based on the MIT open source protocol:
 * <a href="https://github.com/AykutSarac/github-rater/blob/main/LICENSE">AykutSarac/github-rater LICENSE</a>
 * <p>
 * Fine, this is really badly written. I'll come back and revise it later if I have time :)
 */
@Slf4j
@Data
@Accessors(fluent = true)
public class SimpleRank {
    final Statistics statistics;
    final Rating rating;

    public SimpleRank(Statistics statistics) {
        this.statistics = statistics;
        this.rating = new Rating();

        check();
        rate();
    }

    public void check() {
        rating.isBioExist = statistics.user.getBio() != null;

        try {
            rating.isCompanyExist = statistics.user.getCompany() != null;
            rating.isLocationExist = statistics.user.getLocation() != null;
            rating.isBlogExist = statistics.user.getBlog() != null;

            rating.followersCount = statistics.user.getFollowers().size();
        } catch (IOException e) {
            log.error(e.getMessage());
        }

        rating.forksCount = statistics.repositories.stream().mapToDouble(GHRepository::getForksCount).sum();
        rating.starsCount = statistics.repositories.stream().mapToDouble(GHRepository::getStargazersCount).sum();
        rating.repositoriesCount = statistics.repositories.size();
    }

    public void rate() {
        rateBio();
        ratePopularity();
        rateRepositoriesPopularity();
        rateRepositoriesDescription();
        rateWebpages();
        rateBacklinks();
    }

    public void rateBio() {
        if (statistics.user.getBio() == null) return;

        int wordsCount = statistics.user.getBio().split(" ").length;
        int result = wordsCount * 10;

        rating.bioRating = Math.min(result, 100);
    }

    public void ratePopularity() {
        int starRate = statistics.repositories.stream().mapToInt(GHRepository::getStargazersCount).sum() / statistics.repositories.size();

        int rate = 0;
        try {
            rate = statistics.user.getFollowers().size() / statistics.repositories.size() + starRate;
        } catch (IOException e) {
            log.error(e.getMessage());
        }

        int result = rate * 15;
        rating.userPopularity = Math.min(result, 100);
    }

    public void rateRepositoriesPopularity() {
        int totalStars = statistics.repositories.stream().mapToInt(GHRepository::getStargazersCount).sum();
        int totalForks = statistics.repositories.stream().mapToInt(GHRepository::getForksCount).sum();

        double rate = (totalStars + totalForks * 1.2) / statistics.repositories.size();
        double result = (int) Math.round(rate * 16);

        rating.repositoriesPopularity = Math.min(result, 100);
    }

    public void rateRepositoriesDescription() {
        int repositoriesDescriptionCount = statistics.repositories.stream()
                .map(GHRepository::getDescription)
                .filter(Objects::nonNull)
                .map(s -> s.split(" "))
                .filter(s -> s.length > 4)
                .mapToInt(s -> s.length)
                .sum();

        int rate = statistics.repositories.size() / repositoriesDescriptionCount;
        int result = 0;
        if (rate != 0) result = Math.round((float) 100 / rate);

        rating.repositoriesDescriptionRating = Math.min(result, 100);
    }

    public void rateWebpages() {
        int webpageCount = (int) statistics.repositories.stream().filter(r -> r.getHtmlUrl() != null).count();
        int rate = (webpageCount / statistics.repositories.size()) * 100;
        int result = (int) Math.round(rate * 1.8);

        rating.webpagesRating = Math.min(result, 100);
    }

    public void rateBacklinks() {
        int bio = rating.isBioExist ? 1 : 0;
        int loc = rating.isLocationExist ? 1 : 0;
        int blog = rating.isBlogExist ? 1 : 0;
        int company = rating.isCompanyExist ? 1 : 0;

        int result = (bio + loc + blog + company) / 4;
        rating.backlinksRating = result * 100;
    }

    public record Statistics(GHUser user, Collection<GHRepository> repositories) {
    }

    @NoArgsConstructor
    public static class Rating {
        boolean isBioExist;
        boolean isCompanyExist;
        boolean isLocationExist;
        boolean isBlogExist;
        double bioRating;
        double backlinksRating;
        double repositoriesDescriptionRating;
        double webpagesRating;
        double userPopularity;
        double repositoriesPopularity;
        double forksCount;
        double starsCount;
        double repositoriesCount;
        double followersCount;
    }
}
