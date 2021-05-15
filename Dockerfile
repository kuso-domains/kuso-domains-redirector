FROM rust:1.52 as builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kuso-domains-redirector .

CMD ["/app/kuso-domains-redirector"]
