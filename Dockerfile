FROM rust:1.48

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["kuso-domains-redirector"]
