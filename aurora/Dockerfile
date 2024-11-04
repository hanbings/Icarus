FROM debian:bookworm-slim
COPY aurora/target/release/aurora aurora
RUN apt update && apt install -y pkg-config libssl-dev
RUN chmod +x aurora
ENTRYPOINT ["/aurora"]