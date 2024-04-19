# Stage 1 (Builder)
FROM alpine:3.17 AS builder

RUN apk add --no-cache rust cargo postgresql-dev

WORKDIR /app
COPY Cargo.toml ./

# Stage 2 (Final)
FROM alpine:3.17

RUN apk add --no-cache curl

WORKDIR /app
COPY --from=builder /app/target/release/author .

EXPOSE 8080
ENTRYPOINT ["/app/author"]