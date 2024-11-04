FROM debian:bookworm-slim
COPY icarus/target/release/icarus icarus
RUN apt-get install pkg-config libssl-de
RUN chmod +x icarus
ENTRYPOINT ["/icarus"]