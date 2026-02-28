FROM rust:1.75-slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libpq5 \
    poppler-utils \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/infinitebase-backend /app/infinitebase-backend

RUN mkdir -p /app/data /app/uploads

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -fsS http://localhost:8080/health || exit 1

CMD ["/app/infinitebase-backend"]
