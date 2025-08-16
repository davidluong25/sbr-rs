# SBR-RS SaaS Platform
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 app

# Copy binary from builder stage
COPY --from=builder /app/target/release/sbr /usr/local/bin/sbr

# Change ownership to app user
RUN chown -R app:app /usr/local/bin/sbr

USER app

EXPOSE 3000

CMD ["sbr"]