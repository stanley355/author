# Use the Rust official image as a base
FROM rust:latest as build

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

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