FROM --platform=$BUILDPLATFORM golang:1.21 as build
ARG TARGETOS 
ARG TARGETARCH

WORKDIR /app

# COPY go.mod go.sum ./
# RUN go mod download && go mod tidy && go mod verify

COPY go.work go.work.sum ./
COPY avs-aggregator ./avs-aggregator
COPY avs-eigensdk-go ./avs-eigensdk-go

RUN CGO_ENABLED=0 GOOS=$TARGETOS GOARCH=$TARGETARCH go build -o /avs-aggregator avs-aggregator/cmd/main.go

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /avs-aggregator /avs-aggregator
ENTRYPOINT [ "/avs-aggregator"]