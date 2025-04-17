FROM ghcr.io/foundry-rs/foundry:v1.0.0

WORKDIR /app

COPY --chown=foundry:foundry . .
RUN forge build
# RUN forge test

ENTRYPOINT ["forge"]
