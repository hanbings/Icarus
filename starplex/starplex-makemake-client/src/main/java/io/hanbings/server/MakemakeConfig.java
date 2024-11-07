package io.hanbings.server;

import java.util.List;

@SuppressWarnings("SpellCheckingInspection")
public record MakemakeConfig(
        String name,
        String secret,
        List<String> endpoints
) { }
