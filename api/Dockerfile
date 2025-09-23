ARG APP_NAME=autumn-homepage-api

# === Build Rust App ===
FROM rust:1.90-alpine3.20 AS rust_stage
WORKDIR /app

## Build Dependencies
# `musl-dev` required for `cargo build`
# `libressl-dev` required for `cargo build`
# `curl` required to install crate `utoipa-swagger-ui`
RUN apk add --no-cache musl-dev libressl-dev curl \
    && rustup default stable

COPY Cargo.toml Cargo.lock ./
COPY entity ./entity
COPY migration ./migration

RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release

## Build Rust application
COPY src ./src

RUN cargo build --release

# === Run application ===
FROM alpine:3.20
ARG APP_NAME
ENV APP_NAME=${APP_NAME}
WORKDIR /app

RUN apk add --no-cache ca-certificates curl

COPY --from=rust_stage /app/target/release/${APP_NAME} /app

ENV IP="0.0.0.0"
ENV PORT=8000

# Set base log level for `env_logger`, this will be overwritten by .env
ENV RUST_LOG=${APP_NAME}=info

EXPOSE 8000

CMD ["sh", "-c", "./${APP_NAME}"]
