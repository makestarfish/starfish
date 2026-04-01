FROM rust:1.93.1 AS builder

WORKDIR /usr/src/starfish

COPY . .

RUN cargo install --path .

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/starfish /usr/bin/starfish

EXPOSE 3333

CMD ["starfish"]
