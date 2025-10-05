# Get started with a build env with Rust nightly
FROM rust:alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen libressl-dev pkgconfig

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --split --release -vv

FROM rust:alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/server"]
