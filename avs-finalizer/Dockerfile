# syntax=docker/dockerfile:1
ARG ROLLUP_NODE_VERSION=761b67eacf03f2f13ca34202be50e3a3f0c25ef8
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
		cp target/release/main /app/main


FROM debian:stable-slim

WORKDIR /app

RUN apt-get update && apt-get install -y supervisor && rm -rf /var/lib/apt/lists/*

ADD https://raw.githubusercontent.com/mangata-finance/chainspec/e6dbb96641a79a8d77c2ce79c20c1ed598958f00/rollup-holesky-3a07824d8b8a6ad027424629b814a5edb61654cf-raw.json /app/holesky-chainspec.json
COPY --from=builder /app/main /app/finalizer
COPY --from=rollup-node-image /app/node /app/rollup-node

# rollup-node default env vars for connecting to rollup-holesky testnet
ENV ROLLUP_NODE_CHAIN=/app/holesky-chainspec.json \
		ROLLUP_NODE_BOOTNODE=/dns/collator-01-p2p-rollup-holesky.gasp.xyz/tcp/30333/p2p/12D3KooWQVtLSDM8ZuaExjLygBKBjA43gKUZH3wiooxs7Qp7iEQK \
		ROLLUP_NODE_BASE_PATH=/data

# avs-finalizer default env vars for connecting to rollup-holesky testnet
ENV SUBSTRATE_RPC_URL='ws://127.0.0.1:9944' \
		ETH_RPC_URL='https://ethereum-holesky-rpc.publicnode.com' \
		ETH_WS_URL='wss://ethereum-holesky-rpc.publicnode.com' \
		CHAIN_ID='17000' \
		AVS_BLOCK_VALIDATION_PERIOD='100' \
    AVS_DEBOUNCE_RPC='20' \
    AVS_DEPLOYMENT_BLOCK='1233400' \
    AVS_KICK_PERIOD='25' \
    AVS_REGISTRY_COORDINATOR_ADDR='0xfd6A45621DDfeBF94C082e60E0De92aA305a97a1' \
    AVS_RPC_URL='https://avs-aggregator-holesky.gasp.xyz/' \
    AVS_TASK_EXPIRATION='300' \
    AVS_UPDATE_STAKE_PERIOD='150' \
    OPT_IN_AT_STARTUP='true'

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
	command=/app/rollup-node --chain=%(ENV_ROLLUP_NODE_CHAIN)s --bootnodes=%(ENV_ROLLUP_NODE_BOOTNODE)s --base-path=%(ENV_ROLLUP_NODE_BASE_PATH)s --pruning=archive
	stdout_logfile=/dev/stdout
	stdout_logfile_maxbytes=0
	stderr_logfile=/dev/stderr
	stderr_logfile_maxbytes=0
EOF

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
