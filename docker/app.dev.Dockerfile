FROM rust

COPY . .

RUN cargo build

CMD ["cargo", "run"]