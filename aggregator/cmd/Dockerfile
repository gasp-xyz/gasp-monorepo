FROM --platform=$BUILDPLATFORM golang:1.21 as build
ARG TARGETOS 
ARG TARGETARCH

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download && go mod tidy && go mod verify

COPY . .

RUN CGO_ENABLED=0 GOOS=$TARGETOS GOARCH=$TARGETARCH go build -o /aggregator aggregator/cmd/main.go

FROM debian:stable-slim
WORKDIR /app
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /aggregator /aggregator
ENTRYPOINT [ "/aggregator"]