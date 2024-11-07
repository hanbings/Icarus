package io.hanbings.server.starplex;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;

@EnableScheduling
@SpringBootApplication
@SuppressWarnings("SpellCheckingInspection")
public class StarplexApplication {

    public static void main(String[] args) {
        SpringApplication.run(StarplexApplication.class, args);
    }

}
