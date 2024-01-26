FROM debian:bookworm-slim
RUN apt update
RUN apt install -y openssl

WORKDIR /app
COPY ./target/release/main ./

ENTRYPOINT [ "./main" ]