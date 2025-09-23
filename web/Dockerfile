ARG APP_NAME=autumn-homepage
ARG API_URL

# === Generate Tailwindcss ===
FROM oven/bun:1.2.22-alpine AS bun_stage
WORKDIR /app

## Build Dependencies
COPY package.json bun.lock ./

RUN bun i

## Generate CSS
COPY tailwind.config.ts input.css ./
COPY src ./src

RUN bunx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css

# === Compile Rust App ===
# - Use slim because `binstall dioxus-cli` does not support alpine
FROM rust:1.90-slim AS rust_stage
WORKDIR /app

## Build Dependencies
# `pkg-config` required for `cargo build`
# `libssl-dev` required for `cargo build`
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rustup default nightly \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-binstall \
    && cargo binstall dioxus-cli@0.6.3

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo

RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && dx build --release

## Build Rust application
COPY Dioxus.toml ./
COPY assets ./assets
COPY src ./src
COPY --from=bun_stage /app/assets/tailwind.css /app/assets/tailwind.css

ARG API_URL
ENV API_URL=${API_URL}

RUN dx build --release

# === Run application ===
FROM debian:bookworm-slim
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

COPY --from=rust_stage /app/target/dx/${APP_NAME}/release/web/ /app

ENV IP="0.0.0.0"
ENV PORT=8080

EXPOSE 8080

CMD ["sh", "-c", "./server"]
