# # Use the Rust official image as a base
# FROM rust:latest as builder

# # Set the working directory
# WORKDIR /app

# # Copy the project files into the container
# COPY . /app

# # Install diesel CLI for migration
# RUN cargo install diesel_cli --no-default-features --features postgres

# # Build the project with cargo
# RUN cargo build --release --all-features

# # Use a smaller base image for the final container
# FROM debian:bullseye-slim

# # Copy the binary from the previous build stage
# COPY --from=builder /app/target/release/author .

# # RUN apt update && apt install -y openssl libpq-dev pkg-config
# RUN apt update
# RUN apt upgrade libstdc++6
# RUN apt install -y libpq-dev

# EXPOSE 8080

# # Set the startup command
# CMD ["./author"]

FROM ubuntu:22.04
RUN apt-get update && apt-get upgrade -y
RUN apt-get install libpq-dev -y

RUN apt-get install -y -q build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./src/. /app/src/
COPY Cargo.toml /app/
COPY Cargo.lock /app/
RUN cargo build --release --all-features

EXPOSE 8080
ENTRYPOINT /app/target/release/author