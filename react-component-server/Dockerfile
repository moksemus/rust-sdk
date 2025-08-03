# Multi-stage build for Rust application
FROM rust:1.85-slim as builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the entire workspace
COPY . .

# Build the react-component-server binary
RUN cargo build --release --bin react-component-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false mcpuser

# Set working directory
WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/react-component-server /app/react-component-server

# Copy the components directory
COPY --from=builder /app/react-component-server/components /app/components

# Set correct permissions
RUN chown -R mcpuser:mcpuser /app
USER mcpuser

# Expose port (though MCP typically uses stdio)
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV COMPONENTS_DIR=/app/components
ENV MCP_HTTP_PORT=8080

# Run the MCP server with HTTP transport
CMD ["./react-component-server"]