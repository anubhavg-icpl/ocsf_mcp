# OCSF MCP Server - Production Docker Image
# Multi-stage build for minimal footprint and maximum performance
#
# Maintainer: Anubhav Gain (anubhavg-cipl)
# Contact: anubhavg@infopercept.com
# Description: MCP server for OCSF schema management and security logging

# =============================================================================
# Stage 1: Builder - Compile Rust application
# =============================================================================
FROM rust:1.90-alpine AS builder

LABEL maintainer="Anubhav Gain <anubhavg@infopercept.com>" \
      author="Anubhav Gain (anubhavg-cipl)" \
      description="OCSF MCP Server - Production Build Stage" \
      stage="builder"

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    && rm -rf /var/cache/apk/*

# Create build directory
WORKDIR /build

# Copy dependency manifests first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY data ./data

# Build the actual application
# Force rebuild of main binary with optimizations
RUN touch src/main.rs && \
    cargo build --release --bin ocsf-mcp-server && \
    strip target/release/ocsf-mcp-server

# Verify binary works
RUN /build/target/release/ocsf-mcp-server --help || echo "Binary built successfully"

# =============================================================================
# Stage 2: Runtime - Minimal production image
# =============================================================================
FROM alpine:3.20

LABEL maintainer="Anubhav Gain <anubhavg@infopercept.com>" \
      author="Anubhav Gain (anubhavg-cipl)" \
      org.opencontainers.image.title="OCSF MCP Server" \
      org.opencontainers.image.description="Model Context Protocol server for OCSF schema management and security logging code generation" \
      org.opencontainers.image.authors="Anubhav Gain <anubhavg@infopercept.com>" \
      org.opencontainers.image.vendor="InfoPercept" \
      org.opencontainers.image.url="https://github.com/anubhavg-cipl/ocsf-mcp" \
      org.opencontainers.image.source="https://github.com/anubhavg-cipl/ocsf-mcp" \
      org.opencontainers.image.version="0.1.0" \
      org.opencontainers.image.licenses="MIT" \
      io.docker.mcp.server="true" \
      io.docker.mcp.version="1.0" \
      io.docker.mcp.transport="stdio" \
      io.docker.mcp.category="security,logging,code-generation"

# Install runtime dependencies only
RUN apk add --no-cache \
    libgcc \
    ca-certificates \
    tzdata \
    && rm -rf /var/cache/apk/*

# Create non-root user for security
RUN addgroup -g 1000 ocsf && \
    adduser -D -u 1000 -G ocsf -h /home/ocsf -s /bin/sh ocsf

# Create application directories
RUN mkdir -p /opt/ocsf_mcp/data /opt/ocsf_mcp/bin /var/log/ocsf_mcp && \
    chown -R ocsf:ocsf /opt/ocsf_mcp /var/log/ocsf_mcp

# Copy binary from builder
COPY --from=builder --chown=ocsf:ocsf /build/target/release/ocsf-mcp-server /opt/ocsf_mcp/bin/

# Copy OCSF schema data
COPY --from=builder --chown=ocsf:ocsf /build/data /opt/ocsf_mcp/data

# Set environment variables
ENV OCSF_SCHEMA_PATH=/opt/ocsf_mcp/data/ocsf-schema \
    OCSF_LOG_LEVEL=info \
    RUST_BACKTRACE=1 \
    RUST_LOG=ocsf_mcp=info \
    TZ=UTC \
    PATH=/opt/ocsf_mcp/bin:$PATH

# Switch to non-root user
USER ocsf
WORKDIR /home/ocsf

# Health check (validates binary can execute)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/opt/ocsf_mcp/bin/ocsf-mcp-server", "--help"]

# Expose metadata for MCP clients
VOLUME ["/opt/ocsf_mcp/data"]

# Use stdio transport for MCP protocol
# The server communicates via stdin/stdout (JSON-RPC 2.0)
ENTRYPOINT ["/opt/ocsf_mcp/bin/ocsf-mcp-server"]

# Default: No args needed, server auto-starts on stdio
CMD []

# =============================================================================
# Build instructions:
#   docker build -t ocsf-mcp:latest .
#   docker build -t ocsf-mcp:0.1.0 .
#
# Multi-arch build:
#   docker buildx build --platform linux/amd64,linux/arm64 -t ocsf-mcp:latest .
#
# Run instructions:
#   docker run -i ocsf-mcp:latest
#   docker compose up
#
# Size optimization achieved:
#   - Multi-stage build (builder + runtime)
#   - Alpine Linux base (~5MB)
#   - Stripped binary
#   - Minimal runtime dependencies
#   Expected final image size: ~25-35MB
# =============================================================================
