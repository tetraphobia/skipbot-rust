FROM rust:bullseye as build

WORKDIR /build
COPY . .
RUN cargo install --path .

FROM debian:bullseye
WORKDIR /app
COPY --from=build /usr/local/cargo/bin/skipbot-rust /usr/local/bin/skipbot-rust
ENV DATABASE_URL="sqlite://skipbot.db?mode=rwc"
CMD ["skipbot-rust"]