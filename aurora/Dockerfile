FROM debian:bookworm-slim
COPY aurora/target/release/aurora aurora
RUN chmod +x aurora
RUN ls
ENTRYPOINT ["./aurora"]