# FROM ubuntu:22.04 as builder
# RUN apt-get update && apt-get upgrade -y
# RUN apt-get install libpq-dev -y

# RUN apt-get install -y -q build-essential curl
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# WORKDIR /app
# COPY . /app/
# RUN cargo build --release --all-features
# # EXPOSE 8080
# # ENTRYPOINT /app/target/release/author

# FROM rust:1.75 as runner
# WORKDIR /app
# COPY --from=builder /app .
# EXPOSE 8080
# ENTRYPOINT /app/target/release/author
FROM rust:latest as cargo-build

WORKDIR /app

COPY . /app/

RUN cargo build --release --all-features

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM gcr.io/distroless/cc

WORKDIR /runner/
COPY --from=cargo-build /app/target/release/author /runner

ENTRYPOINT /runner/author