FROM rust:1.67 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build 

FROM ubuntu:20.04 as final
RUN apt-get update && apt-get install -y openssl ca-certificates
RUN update-ca-certificates
RUN apt-get install -y libssl-dev
RUN rm -rf /var/lib/apt/lists/*
EXPOSE 8080
ENV RUST_LOG=info
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/debug/r8tes /usr/local/bin/r8tes
CMD ["r8tes"]
