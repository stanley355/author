FROM ubuntu:22.04 as builder

RUN apt-get update && apt-get upgrade -y
RUN apt-get install libpq-dev -y

RUN apt-get install -y -q build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . /app/

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release --all-features

EXPOSE 8080
ENTRYPOINT /app/target/release/author
