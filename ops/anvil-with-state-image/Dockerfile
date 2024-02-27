FROM ghcr.io/foundry-rs/foundry:nightly-2f4b5dbe7f04d974bf99625325200d214089ee66

WORKDIR /app

COPY tests/integration/anvil-state.json /app/anvil-state.json

ENTRYPOINT anvil --host 0.0.0.0 --load-state /app/anvil-state.json