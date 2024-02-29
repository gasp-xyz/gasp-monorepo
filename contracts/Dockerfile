# Use the latest foundry image
# FROM ghcr.io/foundry-rs/foundry
FROM ghcr.io/foundry-rs/foundry:nightly-ac802618e15039b31e464ae6d1fe3ee39f87cefd


# Copy our source code into the container
WORKDIR /app

# Build and test the source code
COPY . .
RUN forge build
ENTRYPOINT ["forge"]
