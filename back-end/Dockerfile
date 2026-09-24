FROM rust:1.97.1-bookworm AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/back-end /usr/local/back-end
CMD ["/usr/local/back-end"]
