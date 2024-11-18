FROM debian:stable-slim

RUN apt-get update -y && apt-get install -y ca-certificates libssl-dev

WORKDIR /app

ENV RUST_LOG=info

ARG BINARY_PATH=target/release/sequencer

COPY ./${BINARY_PATH} ./

ENTRYPOINT [ "./sequencer" ]
