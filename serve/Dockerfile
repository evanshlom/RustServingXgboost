FROM rust:1.88 as builder

WORKDIR /app
COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/eth-gas-predictor /usr/local/bin/eth-gas-predictor

EXPOSE 3000

CMD ["eth-gas-predictor"]