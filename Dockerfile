FROM rust:1.75-slim as builder

WORKDIR /app

# Create blank project
RUN cargo new --bin app
WORKDIR /app/app

# Copy manifests
COPY Cargo.lock Cargo.toml ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source
COPY src ./src

# Build for real
RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/app/target/release/app /usr/local/bin/app

ENV RPC_URL=https://eth.merkle.io

EXPOSE 3000

CMD ["app"]