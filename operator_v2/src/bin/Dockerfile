FROM rust:latest AS builder

RUN update-ca-certificates
RUN apt update
RUN apt install -y protobuf-compiler clang openssl

WORKDIR /app

COPY ./operator_v2 .

ENV SKIP_WASM_BUILD=1
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update
RUN apt install -y openssl

WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/main ./

ENTRYPOINT [ "./main" ]