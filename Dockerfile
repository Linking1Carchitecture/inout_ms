FROM rust:1.24

WORKDIR /ms_in_out
COPY ./ ./

RUN cargo build 

CMD ["cargo run"]