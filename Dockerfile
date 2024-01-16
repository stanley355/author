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

FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app-builder

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

WORKDIR /app-runner

# Copy our build
COPY --from=builder /app-builder/target/x86_64-unknown-linux-musl/release/author ./

RUN apt install -y libpq-dev

EXPOSE 8080

ENTRYPOINT /app-runner/author