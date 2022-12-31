FROM rust:1.65 as builder

WORKDIR /usr/src/ytmp3
COPY . .
# VOLUME . /usr/src/ytmp3
RUN cargo install --path .

# Building debian image on top of rust tooling
FROM alpine:latest
RUN apk update && apk add youtube-dl
COPY --from=builder /usr/local/cargo/bin/ytmp3 /usr/local/bin/ytmp3
# VOLUME .:/usr/local/bin/ytmp3
EXPOSE 8000
ENTRYPOINT ["ytmp3"]
