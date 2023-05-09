# Use the Rust official image as a base
FROM rust:latest as builder

ARG HOST
ARG PORT 
ARG DATABASE_URL
ARG BEARER_TOKEN 

ENV HOST=$HOST 
ENV PORT=$PORT
ENV DATABASE_URL=$DATABASE_URL
ENV BEARER_TOKEN=$BEARER_TOKEN

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Install diesel CLI for migration
RUN cargo install diesel_cli --no-default-features --features postgres

# Build the project with cargo
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:bullseye-slim

# Copy the binary from the previous build stage
COPY --from=builder /app/target/release/author .

RUN apt update && apt install -y openssl libpq-dev pkg-config

# Set the startup command
CMD ["./author diesel migration run && ./author"]
