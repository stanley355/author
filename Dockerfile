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
