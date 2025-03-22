FROM rust:1.85 AS builder
WORKDIR /usr/src/mural-server
COPY . .
RUN cargo install --path ./

FROM debian:bookworm-slim
RUN apt update
COPY --from=builder /usr/local/cargo/bin/mural-server /usr/local/bin/mural-server

ENV RUST_LOG=info
CMD ["mural-server"]
