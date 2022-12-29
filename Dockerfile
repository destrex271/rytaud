FROM rust:1.65

WORKDIR app
COPY . .

RUN cargo install --path .

EXPOSE 8000
VOLUME .:/app

CMD cargo run
