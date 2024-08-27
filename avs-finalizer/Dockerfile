# syntax=docker/dockerfile:1
ARG ROLLUP_NODE_VERSION=bac5e448bbdb30730c9e8ae8f2a34c1c0442724d
FROM mangatasolutions/rollup-node:${ROLLUP_NODE_VERSION} AS rollup-node-image

FROM rust:1.78 AS builder

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

ARG RUST_TOOLCHAIN=1.78
RUN rustup install $RUST_TOOLCHAIN && rustup default $RUST_TOOLCHAIN && \
	rustup target add wasm32-unknown-unknown && \
	rustup component add rust-src rustfmt clippy

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
		cargo build --locked --release && \
		# copy build artifacts to the root directory to avoid issues with accessing cache mount from 2nd stage
		cp target/release/main /app/main


FROM debian:stable-slim

WORKDIR /app

RUN apt-get update && apt-get install -y supervisor && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/main /app/finalizer
COPY --from=rollup-node-image /app/node /app/rollup-node

COPY <<-EOF /etc/supervisor/conf.d/supervisord.conf
	[supervisord]
	nodaemon=true

	[program:finalizer]
	command=/app/finalizer
	stdout_logfile=/dev/stdout
	stdout_logfile_maxbytes=0
	stderr_logfile=/dev/stderr
	stderr_logfile_maxbytes=0

	[program:rollup-node]
	command=/app/rollup-node
	stdout_logfile=/dev/stdout
	stdout_logfile_maxbytes=0
	stderr_logfile=/dev/stderr
	stderr_logfile_maxbytes=0
EOF

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
