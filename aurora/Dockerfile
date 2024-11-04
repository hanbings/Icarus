FROM debian:bookworm-slim
COPY aurora/target/release/aurora aurora
RUN apt-get install pkg-config libssl-dev
RUN chmod +x aurora
ENTRYPOINT ["/aurora"]