# syntax=docker/dockerfile:1
ARG ROLLUP_NODE_VERSION
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
VOLUME /data

RUN apt-get update && \
    apt-get install -y curl wget supervisor wait-for-it ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    mkdir -p /var/log/rollup-node /data && \
    useradd -m -s /bin/bash appuser && \
    chown -R appuser:appuser /app /var/log/rollup-node /data

COPY --chown=appuser:appuser --from=builder /app/main /app/gasp-avs
COPY --chown=appuser:appuser --from=rollup-node-image /app/node /app/rollup-node
COPY --chown=appuser:appuser ./entrypoint.sh /app/entrypoint.sh

# rollup-node default env vars for connecting to rollup-holesky testnet
ENV ROLLUP_NODE_PROCESS_ENABLED=true
		ROLLUP_NODE_CHAIN=/data/chainspec.json \
		RUST_LOG=info,substrate=warn
		# [required] ROLLUP_NODE_BOOTNODE - bootnode address for the rollup-node to connect to
		# [optional] ROLLUP_NODE_CUSTOM_CHAINSPEC_URL - custom chainspec url to use for the rollup-node
		# [optional] ROLLUP_NODE_FORCE_DOWNLOAD_CHAINSPEC - force download chainspec from the ROLLUP_NODE_CUSTOM_CHAINSPEC_URL

# gasp-avs process default configurations
ENV GASP_AVS_PROCESS_ENABLED=true \
		# SUBSTRATE_RPC_URL is used for connecting to embedded in container rollup-node instance
		SUBSTRATE_RPC_URL='ws://127.0.0.1:9944'

COPY <<-EOF /etc/supervisor/conf.d/supervisord.conf
	[supervisord]
	nodaemon=true
	user=appuser

	# Reference: https://github.com/Supervisor/supervisor/issues/712#issuecomment-396553685 
	[program:gasp-avs]
	command = bash -c "wait-for-it ${SUBSTRATE_RPC_URL#*//} -t 300 -- /app/gasp-avs; kill -s SIGINT `cat supervisord.pid`"
	autostart = %(ENV_GASP_AVS_PROCESS_ENABLED)s
	stopasgroup = true
	autorestart = false
	stdout_logfile = /dev/stdout
	stdout_logfile_maxbytes = 0
	stderr_logfile = /dev/stderr
	stderr_logfile_maxbytes = 0
	# Start after rollup-node
	priority = 20

	[program:rollup-node]
	command = bash -c "/app/rollup-node --chain=%(ENV_ROLLUP_NODE_CHAIN)s --bootnodes=%(ENV_ROLLUP_NODE_BOOTNODE)s --base-path=/data --pruning=archive || kill -s SIGINT `cat supervisord.pid`"
	autostart = %(ENV_ROLLUP_NODE_PROCESS_ENABLED)s
	stopasgroup = true
	autorestart = false
	stdout_logfile = /dev/stdout
	stdout_logfile_maxbytes = 0
	stderr_logfile = /dev/stderr
	stderr_logfile_maxbytes = 0
	priority = 10
EOF

USER appuser
CMD ["/app/entrypoint.sh"]
