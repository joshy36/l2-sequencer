FROM rust:1.84-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy entire project
COPY . .

# Build
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/sequencer /usr/local/bin/app

ENV RPC_URL=https://eth.merkle.io

EXPOSE 3000

CMD ["app"]