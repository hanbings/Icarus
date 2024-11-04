FROM debian:bookworm-slim
COPY flora/target/release/flora flora
RUN chmod +x flora
ENTRYPOINT ["./flora"]