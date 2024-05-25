FROM ghcr.io/foundry-rs/foundry:nightly-2f4b5dbe7f04d974bf99625325200d214089ee66

WORKDIR /app

RUN mkdir -p /app/data
COPY tests/integration/anvil-state.json /app/data/anvil-state.json

VOLUME /app/data

ENTRYPOINT anvil --host 0.0.0.0 --state /app/data/anvil-state.json --steps-tracing