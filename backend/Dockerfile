FROM rust:1.74-slim as builder

WORKDIR /usr/src/app

RUN USER=root cargo new --bin chess-engine-api
WORKDIR /usr/src/app/chess-engine-api

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/chess_engine_api*
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash chess

WORKDIR /app

COPY --from=builder /usr/src/app/chess-engine-api/target/release/chess-engine-api /app/chess-engine-api

RUN chown -R chess:chess /app

USER chess

EXPOSE 3001

CMD ["./chess-engine-api"]
