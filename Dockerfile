FROM rust:1.68.2 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder ./target/release/r8tes ./target/release/r8tes   
ENV RUST_LOG=info
CMD ["/target/release/r8tes"]
