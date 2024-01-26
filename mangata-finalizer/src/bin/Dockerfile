FROM debian:stable-slim

RUN apt-get update 
RUN apt-get install -y ca-certificates

WORKDIR /app
COPY ./target/release/main ./

ENTRYPOINT [ "./main" ]