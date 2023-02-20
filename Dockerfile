FROM rust:1.66 as builder

WORKDIR /usr/src

RUN USER=root cargo new ytmp3

COPY Cargo.toml Cargo.lock /usr/src/ytmp3/

WORKDIR /usr/src/ytmp3

# target platform for alpine
COPY src /usr/src/ytmp3/src

# Final build
RUN cargo build --release

# --------------------------------

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y youtube-dl && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && apt-get install -y yt-dlp && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y wget ffmpeg  && wget https://github.com/ytdl-patched/yt-dlp/releases/download/2023.02.17.334/yt-dlp_linux && chmod +x yt-dlp_linux
COPY --from=builder /usr/src/ytmp3/target/release/ytmp3 /usr/local/bin/ytmp3
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/ytmp3"]
