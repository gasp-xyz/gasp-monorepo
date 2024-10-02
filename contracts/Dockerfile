FROM ghcr.io/foundry-rs/foundry:nightly-c2e529786c07ee7069cefcd4fe2db41f0e46cef6

# Copy our source code into the container
WORKDIR /app

RUN mkdir /data

# Build and test the source code
COPY . .
RUN forge build

ENTRYPOINT ["forge"]
