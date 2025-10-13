FROM rust:alpine AS chef

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen libressl-dev pkgconfig

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
RUN cargo install --locked cargo-chef

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo leptos build -P --split --release -vv

FROM rustlang/rust:nightly-alpine as runner
WORKDIR /app

COPY --from=builder /build/target/release/server .
COPY --from=builder /build/target/site site/
COPY --from=builder /build/Cargo.toml .

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/server"]
