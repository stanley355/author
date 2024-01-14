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

# FROM ubuntu:22.04
# RUN apt-get update && apt-get upgrade -y
# RUN apt-get install libpq-dev -y

# RUN apt-get install -y -q build-essential curl
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# WORKDIR /app
# COPY ./src/. /app/src/
# COPY Cargo.toml /app/
# COPY Cargo.lock /app/
# RUN cargo build --release --all-features

# EXPOSE 8080
# ENTRYPOINT /app/target/release/author

FROM ubuntu:22.04 as OS_BUILDER
RUN apt-get update && apt-get update && apt-get install -y wget
WORKDIR /tmp
RUN wget https://go.dev/dl/go1.21.1.linux-amd64.tar.gz
RUN tar -xvf go1.21.1.linux-amd64.tar.gz
RUN mv go /usr/local
RUN GOBIN=/usr/local/bin/ /usr/local/go/bin/go install github.com/canonical/chisel/cmd/chisel@latest
WORKDIR /rootfs
RUN chisel cut --release ubuntu-22.04 --root /rootfs \
    base-files_base \
    base-files_release-info \
    ca-certificates_data \
    libgcc-s1_libs \
    libc6_libs

FROM rust:1.72.1 as APP_PLANNER
WORKDIR /usr/local/src
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.72.1 as APP_CACHER
WORKDIR /usr/local/src
RUN cargo install cargo-chef
COPY --from=APP_PLANNER /usr/local/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.72.1 as APP_BUILDER
COPY . /usr/local/src
WORKDIR /usr/local/src
COPY --from=APP_CACHER /usr/local/src/target target
COPY --from=APP_CACHER $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM scratch
COPY --from=OS_BUILDER /rootfs /
COPY --from=APP_BUILDER /usr/local/src/target/release/rust-axum /usr/local/bin/rust-axum
WORKDIR /usr/local/bin

ENTRYPOINT [ "./rust-axum" ]

