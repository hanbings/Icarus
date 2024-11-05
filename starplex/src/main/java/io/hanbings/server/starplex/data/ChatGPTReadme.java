package io.hanbings.server.starplex.data;

import org.jetbrains.annotations.Nullable;

import java.util.List;

public record ChatGPTReadme(
        @Nullable String description,
        @Nullable String country,
        @Nullable String blog,
        @Nullable List<String> language,
        int score
) { }
