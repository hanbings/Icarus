FROM debian:bookworm-slim
COPY flora/target/release/flora flora
RUN apt update && apt install -y pkg-config libssl-dev
RUN chmod +x flora
ENTRYPOINT ["/flora"]