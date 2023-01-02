FROM rust:1.65 as builder

WORKDIR /usr/src

RUN USER=root cargo new ytmp3

COPY Cargo.toml Cargo.lock /usr/src/ytmp3/

WORKDIR /usr/src/ytmp3

# target platform for alpine
COPY src /usr/src/ytmp3/src

# Final build
RUN cargo build --release

# --------------------------------

FROM debian:11-slim
COPY --from=builder /usr/src/ytmp3/target/release/ytmp3 /usr/local/bin/ytmp3
ENTRYPOINT ["/usr/local/bin/ytmp3"]
