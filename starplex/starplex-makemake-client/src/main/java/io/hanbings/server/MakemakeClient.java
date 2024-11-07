package io.hanbings.server;

import com.google.gson.Gson;
import okhttp3.MediaType;
import okhttp3.OkHttpClient;
import okhttp3.RequestBody;

import java.io.IOException;

@SuppressWarnings("SpellCheckingInspection")
public record MakemakeClient(MakemakeConfig config) {
    static Gson gson = new Gson();
    static OkHttpClient httpClient = new OkHttpClient();

    public void push(String channel, String value) throws IOException {
        String url = String.format("%s/message/%s/push", config.endpoints().getFirst(), channel);
        PushMessage message = new PushMessage(channel, value);

        var res = httpClient.newCall(
                        new okhttp3.Request.Builder()
                                .url(url)
                                .addHeader("Content-Type", "application/json")
                                .addHeader("Authorization", String.format("Bearer %s", config.secret()))
                                .post(RequestBody.create(gson.toJson(message), MediaType.parse("application/json")))
                                .build())
                .execute();

        if (!res.isSuccessful()) {
            throw new IOException(res.message());
        }

        res.close();
    }

    public String pop(String channel) throws IOException {
        String tokenUrl = String.format("%s/message/%s/pop", config.endpoints().getFirst(), channel);
        var res = httpClient.newCall(
                        new okhttp3.Request.Builder()
                                .url(tokenUrl)
                                .addHeader("Content-Type", "application/json")
                                .addHeader("Authorization", String.format("Bearer %s", config.secret()))
                                .get()
                                .build())
                .execute();

        if (!res.isSuccessful()) throw new IOException(res.message());
        if (res.body() == null) return null;

        var token = gson.fromJson(res.body().string(), PopToken.class);
        res.close();

        var messageUrl = String.format("%s/pop/%s", config.endpoints().getFirst(), token.token);
        res = httpClient.newCall(
                        new okhttp3.Request.Builder()
                                .url(messageUrl)
                                .addHeader("Content-Type", "application/json")
                                .addHeader("Authorization", String.format("Bearer %s", config.secret()))
                                .get()
                                .build())
                .execute();

        if (!res.isSuccessful()) throw new IOException(res.message());
        if (res.body() == null) return null;

        var message = gson.fromJson(res.body().string(), PopMessage.class);

        res.close();
        return message.data;
    }

    public record PushMessage(String channel, String value) {
    }

    public record PopToken(String token) {
    }

    public record PopMessage(String data) {
    }
}