FROM rust:latest

WORKDIR /usr/src/app
COPY ../engine .
COPY ../order_book .

WORKDIR /usr/src/app/service
COPY . .

RUN cargo install --path ./service

CMD ["service"]
