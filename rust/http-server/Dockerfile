FROM rust:latest

RUN mkdir -p /myhttpd/root

WORKDIR /myhttpd

COPY src ./src
COPY Cargo.toml ./
COPY settings.yml ./

RUN cargo update
RUN cargo build --release

EXPOSE 80

CMD ["./target/release/http-server"]
