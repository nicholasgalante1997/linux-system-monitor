# Create a base layer with a small Linux image
FROM ubuntu:24.10 AS base

# Create a layer where we setup Rust
FROM base AS rust_local

# Install dependencies needed for building Rust application
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup update stable \
    && rustup default stable

# Create a layer for setting up the workspace
FROM rust_local AS workspace

# Create our working directory
RUN mkdir -p /home/app/system-monitor

# Move over to our working directory
WORKDIR /home/app/system-monitor

# Copy our server dependencies
COPY Cargo.* ./

# Copy our source code
COPY ./src/ ./src/
COPY ./public/ ./public/

# Create a layer to build our app
FROM workspace AS builder

# Build our production server
RUN . $HOME/.cargo/env && cargo build --release

# Create a layer to run our app, we can reuse our teeny tiny linux image
FROM base AS runner

# Install necessary runtime dependencies (note, we don't even need Rust!)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create working directory
RUN mkdir -p /home/app/system-monitor
WORKDIR /home/app/system-monitor

# Copy Rust binary + public folder
COPY --from=builder /home/app/system-monitor/target/release/system-monitor /home/app/system-monitor/system-monitor
COPY --from=builder /home/app/system-monitor/public/ /home/app/system-monitor/public/

# Ensure the Rust binary has execution permissions
RUN chmod +x /home/app/system-monitor/system-monitor

# Expose port 8080
EXPOSE 8080

# Set environment variables
ENV PORT=8080

# Run the Rust binary
CMD ["./system-monitor"]

HEALTHCHECK --interval=30s --timeout=10s --retries=3 CMD curl --fail http://localhost:8080/health || exit 1