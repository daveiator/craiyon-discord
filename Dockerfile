FROM rust:alpine as builder
WORKDIR /usr/src/craiyon-discord
COPY . . 
RUN apk update && apk upgrade && apk add ca-certificates pkgconfig musl-dev openssl openssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/craiyon-discord /usr/local/bin/craiyon-discord
RUN apt-get update && apt-get upgrade -y && apt-get install ca-certificates libssl-dev pkg-config musl-dev -y && rm -rf /var/lib/apt/lists/*
CMD ["craiyon-discord"]
