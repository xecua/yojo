# stage 1: build
FROM rust:1.43 as builder
WORKDIR /app
RUN apt install default-libmysqlclient-dev && \
    cargo install diesel_cli --no-default-features --features mysql
COPY . .
RUN cargo install --path .

# stage 2: run
FROM debian:buster-slim
WORKDIR /app
COPY ./migrations /app/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
RUN apt update && apt install -y default-libmysqlclient-dev libssl-dev
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/myapp
