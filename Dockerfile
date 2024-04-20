FROM ubuntu:22.04 as builder

RUN apt-get update && apt-get upgrade -y
RUN apt-get install libpq-dev build-essential curl -y

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . /app/

RUN cargo build --release --all-features

FROM ubuntu:22.04 as runner 

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/author .

RUN echo "LIB LIB LIB LIB LIB"
RUN find / -name libpq.so.5

EXPOSE 8080
# ENTRYPOINT /app/target/release/author
CMD ["./author"]