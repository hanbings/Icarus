package io.hanbings.server.starplex.config;

import lombok.Data;
import lombok.experimental.Accessors;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import java.util.List;

@Data
@Component
@Accessors(fluent = true)
@SuppressWarnings("SpellCheckingInspection")
public class MakemakeConfig {
    @Value("${makemake.name}")
    String name;
    @Value("${makemake.secret}")
    String secret;
    @Value("${makemake.endpoints}")
    List<String> endpoints;
}
