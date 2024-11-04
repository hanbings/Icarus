FROM debian:bookworm-slim
COPY makemake/target/release/makemake makemake
RUN apt update && apt install -y pkg-config libssl-dev
RUN chmod +x makemake
ENTRYPOINT ["/makemake"]