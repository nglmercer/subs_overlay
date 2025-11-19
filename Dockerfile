# Multi-stage build for subtitle-overlay
FROM rust:1.75 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libx11-dev \
    libfontconfig1-dev \
    libgtk-3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY ui/ ui/
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libx11-6 \
    libfontconfig1 \
    libgtk-3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 subtitleuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/slint-subtitles /usr/local/bin/subtitle-overlay

# Create config directory
RUN mkdir -p /config && chown subtitleuser:subtitleuser /config

USER subtitleuser

# Expose API port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV SUBTITLE_OVERLAY_SERVER__HOST=0.0.0.0
ENV SUBTITLE_OVERLAY_SERVER__PORT=8080

# Default command
CMD ["subtitle-overlay", "api"]
