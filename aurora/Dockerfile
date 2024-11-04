FROM alpine:3.14
COPY aurora/target/release/aurora aurora
RUN chmod +x aurora
ENTRYPOINT ["./aurora"]