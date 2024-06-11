# Use the latest foundry image
# FROM ghcr.io/foundry-rs/foundry
FROM ghcr.io/foundry-rs/foundry:nightly-e5318c3054e5f883d1467da9fae5d29567a03d43


# Copy our source code into the container
WORKDIR /app

# Build and test the source code
COPY . .
RUN forge build
ENTRYPOINT ["forge"]
