# 1. Build Stage
FROM rust:1.86.0 AS builder

WORKDIR /app

ENV SQLX_OFFLINE=true

COPY . .
RUN cargo build --release

# 2. Runtime Stage
FROM debian:bookworm-slim

# Copy compiled binary
COPY --from=builder /app/target/release/relay /app/app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

CMD ["/app/app"]
