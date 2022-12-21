FROM rust:1.65

WORKDIR app
COPY . .

RUN cargo install --path

CMD["cargo run"]
