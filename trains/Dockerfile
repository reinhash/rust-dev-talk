FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/trains"]