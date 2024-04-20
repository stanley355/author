FROM ubuntu:22.04 as builder

RUN apt-get update && \
    apt-get install -y libpq-dev build-essential curl && \
    apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM ubuntu:22.04 as runner

RUN apt-get update && \
    apt-get install -y libpq-dev

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/author /app

EXPOSE 8080
ENTRYPOINT /app/author