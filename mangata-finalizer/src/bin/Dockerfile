FROM rust:latest AS builder

RUN apt update
RUN apt install -y protobuf-compiler clang
RUN update-ca-certificates

WORKDIR /app

COPY ./mangata-finalizer .

ENV SKIP_WASM_BUILD=1
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update
RUN apt install -y openssl

WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/main ./

ENTRYPOINT [ "./main" ]