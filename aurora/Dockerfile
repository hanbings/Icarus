FROM debian:bookworm-slim
COPY aurora/target/release/aurora aurora
RUN sudo apt-get install pkg-config libssl-dev
RUN chmod +x aurora
ENTRYPOINT ["/aurora"]