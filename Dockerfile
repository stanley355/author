# Stage 1: Build Stage
FROM ubuntu:22.04 as builder

# Install necessary dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    build-essential \
    curl

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy project files
COPY . /app/

RUN cargo build --release --all-features
# Build the project

# Stage 2: Runtime Stage
FROM ubuntu:22.04 as runner

# Copy built artifacts from the builder stage
COPY --from=builder /app/target/release/author /app/target/release/author

# Expose port
EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["/app/target/release/author"]
