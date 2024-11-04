FROM debian:bookworm-slim
COPY icarus/target/release/icarus icarus
RUN chmod +x icarus
ENTRYPOINT ["/icarus"]