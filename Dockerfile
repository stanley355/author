# Use the Rust official image as a base
FROM ubuntu:latest

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Ubuntu
RUN apt-get update && apt-get install -y curl build-essential libssl-dev pkg-config libpq-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install diesel CLI for migration
RUN cargo install diesel_cli --no-default-features --features postgres

# Build the project with cargo
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Copy the binary from the previous build stage
COPY --from=build /app/target/release/author .

# Set the startup command
CMD ["bash", "-c", "./author diesel migration run && ./author"]
