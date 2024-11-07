package io.hanbings.server;

import java.util.List;

@SuppressWarnings("SpellCheckingInspection")
public record MakemakeStatus(
        String leader,
        List<String> channels
) {
}
