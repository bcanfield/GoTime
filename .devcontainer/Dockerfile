FROM rust:bookworm

# Install system dependencies and Node.js
RUN apt-get update && apt-get install -y \
      curl \
      ca-certificates \
      binaryen \
      build-essential \
      && rm -rf /var/lib/apt/lists/*

# Install Node.js (example using Node 18)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get update && apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Install SpacetimeDB as root to ensure proper permissions
USER root
RUN curl -sSfL https://install.spacetimedb.com | bash -s -- --yes

# Make sure the SpacetimeDB binary is available in the system PATH
ENV PATH="/usr/local/bin:${PATH}"

# Copy the SpacetimeDB binary to a location accessible to all users
RUN mkdir -p /usr/local/bin && \
    cp /root/.local/bin/spacetime /usr/local/bin/ && \
    chmod +x /usr/local/bin/spacetime

# Create a non-root user 'node' and switch to it
RUN useradd -ms /bin/bash node
USER node

# (Optional) Update PATH for Cargo binaries if needed
# The official rust image installs rustup; the cargo bin may be in /usr/local/cargo/bin.
# Adjust the PATH below if you use per-user installations.
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Add the wasm target for Rust
RUN rustup target add wasm32-unknown-unknown

# Ensure the SpacetimeDB binary remains in the PATH for interactive shells
RUN echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
