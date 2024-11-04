FROM debian:bookworm-slim
COPY makemake/target/release/makemake makemake
RUN apt update && apt install pkg-config libssl-dev
RUN chmod +x makemake
ENTRYPOINT ["/makemake"]