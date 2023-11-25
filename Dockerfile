FROM docker.io/rust:1.72.1-bookworm as builder

WORKDIR /build

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Ensure we static link
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
RUN mv target/release/rustycloud app

FROM scratch

COPY --from=builder /build/app /app

CMD [ "./app" ]
