# Build from git ```sudo docker build https://github.com/acottis/rustycloud.git#main -t rustycloud-tag```
# Run on port 8080 and detach from terminal ```sudo docker run -dp 80:8080 rustycloud-tag```


FROM rust:1.56-slim-buster

# RUN apt-get update; apt-get --assume-yes install curl

# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=1 \
#     CMD curl -f http://localhost:8080 || exit 1

ENV PORT=8080

WORKDIR /hello-from-rustia
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./src ./src

RUN cargo build --release

CMD cargo run --release