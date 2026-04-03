FROM rust:1.93.1 AS builder

WORKDIR /usr/src/starfish

COPY . .

RUN cargo install --path .
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/starfish /usr/bin/starfish
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/bin/sqlx
COPY --from=builder /usr/src/starfish/migrations migrations

EXPOSE 3333

CMD ["starfish"]
