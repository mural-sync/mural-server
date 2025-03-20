FROM rust:1.85 AS builder
WORKDIR /usr/src/mural_server
COPY . .
RUN cargo install --path ./

FROM debian:bookworm-slim
RUN apt update
COPY --from=builder /usr/local/cargo/bin/mural_server /usr/local/bin/mural_server

CMD ["mural_server"]
