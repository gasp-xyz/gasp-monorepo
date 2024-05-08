# Use the latest foundry image
FROM --platform=linux/amd64 ghcr.io/foundry-rs/foundry

# Copy our source code into the container
WORKDIR /app

# Build and test the source code
COPY . .
RUN forge build
ENTRYPOINT ["forge"]
