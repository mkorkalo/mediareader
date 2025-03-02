
FROM rust:1.84.0 as builder
WORKDIR /usr/src/mediadownloader
COPY . .
RUN cargo install --path .
FROM debian:bookworm-slim as prog
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mediadownloader /usr/local/bin/mediadownloader
RUN ls -la /usr/local/bin/mediadownloader
CMD ["/usr/local/bin/mediadownloader"]
