FROM rust:1.81-slim as builder

RUN apt-get update && \
    apt-get install -y libpq-dev build-essential curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM ubuntu:22.04 as runner

RUN apt-get update && \
    apt-get install -y libpq-dev build-essential curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/author /app/author

EXPOSE 8080
ENTRYPOINT /app/author