package io.hanbings.server;

import com.google.gson.Gson;
import okhttp3.OkHttpClient;

@SuppressWarnings("SpellCheckingInspection")
public record MakemakeClient(MakemakeConfig config) {
    static Gson gson = new Gson();
    static OkHttpClient httpClient = new OkHttpClient();

    public void push(String channel, String value) {

    }

    public String pop(String channel) {
        return null;
    }

    public <T> void push(String channel, T value) {
        String json = gson.toJson(value);
    }

    public <T> T pop(String channel, Class<T> clazz) {
        return null;
    }

    String discoveryLeader() {
        return null;
    }
}