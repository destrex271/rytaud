FROM rust:1.65 as builder

WORKDIR /usr/src/ytmp3
COPY . .
# VOLUME . /usr/src/ytmp3
RUN cargo install --path .

# Building debian image on top of rust tooling
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y youtube-dl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ytmp3 /usr/local/bin/ytmp3
# VOLUME .:/usr/local/bin/ytmp3
EXPOSE 8000
ENTRYPOINT ["ytmp3"]
