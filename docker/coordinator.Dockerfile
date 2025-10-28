# BUILD
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build -p coordinator --release

# RUN
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/coordinator .
ENV RUST_LOG=info
EXPOSE 8080
CMD ["./coordinator"]