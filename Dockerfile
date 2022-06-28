FROM rust:alpine as builder
WORKDIR /usr/src/craiyon-discord
COPY . .
RUN apk update && apk upgrade && apk add musl-dev openssl openssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/craiyon-discord /usr/local/bin/craiyon-discord
CMD ["craiyon-discord"]
