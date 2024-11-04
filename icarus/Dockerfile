FROM alpine:3.14
COPY icarus/target/release/icarus icarus
RUN chmod +x icarus
ENTRYPOINT ["./icarus"]