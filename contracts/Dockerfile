FROM ghcr.io/foundry-rs/foundry:nightly-1f22112b4e9586e39924ab851c055088c3973683

# Copy our source code into the container
WORKDIR /app

# Build the source code
COPY . .
RUN forge build

ENTRYPOINT ["forge"]
