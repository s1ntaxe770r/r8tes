FROM rust:1.68.2 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/r8tes ./target/release/r8tes
CMD ["/target/release/r8tes"]
