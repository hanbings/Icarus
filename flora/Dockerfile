FROM alpine:3.14
COPY flora/target/release/flora flora
RUN chmod +x flora
ENTRYPOINT ["./flora"]