FROM debian:stable-slim

RUN apt-get update 
RUN apt-get install -y ca-certificates

WORKDIR /app

ARG BINARY_PATH=target/release/main

COPY ./${BINARY_PATH} ./

ENTRYPOINT [ "./main" ]