FROM rust:1.88-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sub2api-gateway /usr/local/bin/sub2api-gateway
COPY .env.example /app/.env.example
EXPOSE 8088
CMD ["sub2api-gateway"]
