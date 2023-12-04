FROM golang:1.21 as build

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download && go mod tidy && go mod verify

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o /aggregator aggregator/cmd/main.go

FROM debian:buster-slim
WORKDIR /app
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /aggregator /aggregator
ENTRYPOINT [ "/aggregator"]