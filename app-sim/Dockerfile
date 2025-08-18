# Build stage
FROM rust:1.82-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/app-sim /usr/local/bin/app-sim
ENV OTLP_GRPC_ENDPOINT=http://otel-collector:4317
ENTRYPOINT ["/usr/local/bin/app-sim"]



