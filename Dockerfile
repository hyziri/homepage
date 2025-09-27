ARG APP_NAME=autumn-homepage

# === Generate Tailwindcss ===
FROM oven/bun:1.2.22-alpine AS bun_stage
WORKDIR /app

# Build Dependencies
COPY package.json bun.lock ./

RUN bun i

# Generate CSS
COPY tailwind.config.ts tailwind.css ./
COPY src ./src

RUN bunx @tailwindcss/cli -i ./tailwind.css -o ./assets/tailwind.css

# === Compile Rust App ===
# Use debian bookworm slim because `binstall dioxus-cli` does not support alpine
FROM rust:1.90-slim AS rust_stage
WORKDIR /app

# Build Dependencies
# `pkg-config` required for `cargo build`
# `libssl-dev` required for `cargo build`
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

RUN rustup default stable \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-binstall

RUN cargo binstall dioxus-cli@0.6.3

# Build Rust dependencies
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY entity ./entity
COPY migration ./migration

RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && dx build --release

# Build Rust application
COPY assets ./assets
COPY src ./src
COPY --from=bun_stage /app/assets/tailwind.css /app/assets/tailwind.css

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
ENV DATABASE_URL="sqlite://data/db.sqlite?mode=rwc"

EXPOSE 8080

CMD ["sh", "-c", "./server"]
