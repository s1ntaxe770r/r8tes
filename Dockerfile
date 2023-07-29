FROM rust:1.67 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye

RUN apt-get update \
 && apt-get install -y --force-yes --no-install-recommends ca-certificates libssl-dev \
 && apt-get clean \
 && apt-get autoremove \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder ./target/release/r8tes ./target/release/r8tes   
ENV RUST_LOG=info
CMD ["/target/release/r8tes"]
