FROM debian:bookworm-slim
COPY icarus/target/release/icarus icarus
RUN sudo apt-get install pkg-config libssl-de
RUN chmod +x icarus
ENTRYPOINT ["/icarus"]