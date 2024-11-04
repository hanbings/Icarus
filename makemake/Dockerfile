FROM debian:bookworm-slim
COPY makemake/target/release/makemake makemake
RUN sudo apt-get install pkg-config libssl-dev
RUN chmod +x makemake
ENTRYPOINT ["/makemake"]