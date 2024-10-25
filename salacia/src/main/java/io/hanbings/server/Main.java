package io.hanbings.server;

import com.google.gson.ExclusionStrategy;
import com.google.gson.FieldAttributes;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoDatabase;
import org.bson.Document;
import org.kohsuke.github.*;
import org.tinylog.Logger;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;

@SuppressWarnings("SpellCheckingInspection")
public class Main {
    public static void main(String[] args) throws IOException {
        String token = System.getenv("GITHUB_TOKEN");
        String uri = System.getenv("DATABASE");
        if (token == null) throw new RuntimeException("GITHUB_TOKEN is not set");
        if (uri == null) throw new RuntimeException("DATABASE is not set");

        Gson gson = new GsonBuilder()
                .setExclusionStrategies(new NodeIdExclusionStrategy())
                .create();

        GitHub github = new GitHubBuilder().withOAuthToken(token).build();
        Logger.info("Connected to GitHub");
        GHPersonSet<GHUser> followers = github.getMyself().getFollowers();

        try (MongoClient client = MongoClients.create(uri)) {
            MongoDatabase database = client.getDatabase("github");

            GHUser myself = github.getMyself();
            database.getCollection("hanbings-followers-repositories")
                    .insertOne(Document.parse(gson.toJson(getRepositories(myself))));

            int count = 0;
            for (GHUser follower : followers) {
                database.getCollection("hanbings-followers-repositories")
                        .insertOne(Document.parse(gson.toJson(getRepositories(follower))));

                count++;
                if (count % 10 == 0) {
                    Logger.info("Processed {} followers", count);
                }
            }
        }
    }

    public static Salacia getRepositories(GHUser user) throws IOException {
        List<GHRepository> repositories = new ArrayList<>();

        for (GHRepository repository : user.getRepositories().values()) {
            if (repository.isFork()) continue;
            if (!repository.getOwner().equals(user)) continue;

            repositories.add(repository);
        }

        return new Salacia(user, repositories);
    }

    record Salacia(
            GHUser user,
            Collection<GHRepository> repository
    ) {
    }

    static class NodeIdExclusionStrategy implements ExclusionStrategy {
        @Override
        public boolean shouldSkipField(FieldAttributes fieldAttributes) {
            return fieldAttributes.getName().equals("nodeId");
        }

        @Override
        public boolean shouldSkipClass(Class<?> clazz) {
            return false;
        }
    }
}