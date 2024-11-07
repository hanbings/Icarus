package io.hanbings.server.starplex.config;

import io.hanbings.server.MakemakeClient;
import io.hanbings.server.MakemakeConfig;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.util.List;

@Configuration
@SuppressWarnings("SpellCheckingInspection")
public class MakemakeClientConfig {
    @Bean
    public MakemakeClient makemakeClient(io.hanbings.server.starplex.config.MakemakeConfig config) {
        return new MakemakeClient(
                new MakemakeConfig(
                        config.name,
                        config.secret,
                        List.of(config.endpoint)
                )
        );
    }
}
