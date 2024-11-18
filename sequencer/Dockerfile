FROM debian:stable-slim

RUN apt-get update 
RUN apt-get install -y ca-certificates

WORKDIR /app

ENV RUST_LOG=info

ARG BINARY_PATH=target/release/main

COPY ./${BINARY_PATH} ./

ENTRYPOINT [ "./main" ]
