# syntax=docker/dockerfile:1
FROM rust:1.82 AS builder

WORKDIR /app

RUN set -eux && \
		apt-get -y update; \
		apt-get install -y --no-install-recommends \
		libssl-dev make cmake graphviz clang libclang-dev llvm \
		git pkg-config curl time rhash ca-certificates zstd \
		python3 python3-pip lsof ruby ruby-bundler git-restore-mtime xz-utils unzip gnupg protobuf-compiler && \
		# Install mold linker
		curl -L https://github.com/rui314/mold/releases/download/v2.31.0/mold-2.31.0-$(uname -m)-linux.tar.gz \
		| tar -C /usr/local --strip-components=1 -xzf - && ln -sf /usr/local/bin/mold "$(realpath /usr/bin/ld)" && \
		# apt clean up
		apt-get autoremove -y && apt-get clean && rm -rf /var/lib/apt/lists/*


COPY rust-toolchain.toml /app/rust-toolchain.toml

RUN rustup show && \
		rustup target add wasm32-unknown-unknown && \
		rustup component add rust-src rustfmt clippy

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
		cargo build --locked --release && \
		# copy build artifacts to the root directory to avoid issues with accessing cache mount from 2nd stage
		cp target/release/sequencer /app/sequencer-bin \
		cp target/release/ferry-withdrawal /app/ferry-withdrawal-bin \
		cp target/release/ferry-deposit /app/ferry-deposit


FROM debian:stable-slim
RUN apt update -y && apt install -y libssl-dev ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/sequencer-bin /app/sequencer
COPY --from=builder /app/ferry-withdrawal-bin /app/ferry-withdrawal
COPY --from=builder /app/ferry-deposti-bin /app/ferry-deposit

CMD ["/app/sequencer"]
