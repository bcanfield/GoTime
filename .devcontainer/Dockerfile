FROM mcr.microsoft.com/devcontainers/typescript-node:bookworm

# Install dependencies
RUN apt-get update && apt-get install -y \
      curl \
      ca-certificates \
      binaryen \
      build-essential \
      && rm -rf /var/lib/apt/lists/*

# Install SpacetimeDB as root to ensure proper permissions
USER root
RUN curl -sSfL https://install.spacetimedb.com | bash -s -- --yes

# Make sure the SpacetimeDB binary is available in the system PATH
ENV PATH="/usr/local/bin:${PATH}"

# Copy the SpacetimeDB binary to a location accessible to all users
RUN mkdir -p /usr/local/bin && \
    cp /root/.local/bin/spacetime /usr/local/bin/ && \
    chmod +x /usr/local/bin/spacetime

# Switch to the default non-root user
USER node

# Install Rust using rustup as the non-root user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update PATH to include Cargo binaries
ENV PATH="/home/node/.cargo/bin:${PATH}"

# Use Cargo (via rustup-installed Rust) to install an example package
# RUN cargo install cargo-audit
RUN rustup target add wasm32-unknown-unknown

# Ensure PATH includes the SpacetimeDB executable location for interactive shells
RUN echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
