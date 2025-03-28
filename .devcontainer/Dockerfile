FROM ubuntu:latest

# Install system dependencies and tools
RUN apt-get update && apt-get install -y \
      curl \
      ca-certificates \
      binaryen \
      build-essential \
      wget \
      git \
      gnupg \
      lsb-release \
      && rm -rf /var/lib/apt/lists/*

# Install Node.js (example: Node 18)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get update && apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user "node"
RUN useradd -ms /bin/bash node

# Install SpacetimeDB as root to ensure proper permissions
USER root
RUN curl -sSfL https://install.spacetimedb.com | bash -s -- --yes

# Ensure the SpacetimeDB binary is accessible
ENV PATH="/usr/local/bin:${PATH}"
RUN mkdir -p /usr/local/bin && \
    cp /root/.local/bin/spacetime /usr/local/bin/ && \
    chmod +x /usr/local/bin/spacetime

# Switch to non-root user "node"
USER node

# Install Rust using rustup (runs as non-root)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update PATH to include Cargo binaries
ENV PATH="/home/node/.cargo/bin:${PATH}"

# Add the wasm target for Rust
RUN rustup target add wasm32-unknown-unknown

# Ensure interactive shells pick up the SpacetimeDB binary location
RUN echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
