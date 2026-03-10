FROM rust:latest

WORKDIR /usr/src/app
COPY ../order_book .

WORKDIR /usr/src/app/mdp
COPY . .

RUN cargo install --path ./mdp

CMD ["mdp"]
