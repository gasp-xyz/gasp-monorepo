FROM ghcr.io/foundry-rs/foundry:nightly-2fe902962cd76846f3fb02c24576789ee82b35d5

# Copy our source code into the container
WORKDIR /app

# Build the source code
COPY . .
RUN forge build

ENTRYPOINT ["forge"]
