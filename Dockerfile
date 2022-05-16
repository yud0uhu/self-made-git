FROM rust

WORKDIR /usr/src/self-made-git
COPY . .

RUN cargo install --path .

CMD ["self-made-git"]