FROM debian:bookworm-slim
COPY makemake/target/release/makemake makemake
RUN chmod +x makemake
ENTRYPOINT ["./makemake"]