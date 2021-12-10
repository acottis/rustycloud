FROM rust:1.56-slim-buster

WORKDIR /hello-from-rustia
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release

CMD cargo run --release