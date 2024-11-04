FROM debian:bookworm-slim
COPY icarus/target/release/icarus icarus
RUN apt update && apt install -y pkg-config libssl-dev
RUN chmod +x icarus
ENTRYPOINT ["/icarus"]