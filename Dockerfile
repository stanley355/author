# Stage 1: Builder
FROM rust:slim as builder

# Install dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev build-essential curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust (assuming this is necessary beyond the base rust image)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy source code
WORKDIR /app
COPY . /app/

# Build
RUN cargo build --release --all-features

# Remove unnecessary build files
RUN rm -rf /usr/local/cargo/git /usr/local/cargo/registry

# Stage 2: Runtime
FROM debian:buster-slim

# Install runtime libraries
RUN apt-get update && \
    apt-get install -y libpq5 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/author /app/author

# Set the working directory
WORKDIR /app

# Expose and command
EXPOSE 8080
CMD ["./author"]