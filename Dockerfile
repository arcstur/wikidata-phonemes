FROM lukemathwalker/cargo-chef:latest-rust-1.78 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml .
COPY Cargo.lock .
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml .
COPY Cargo.lock .
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime
EXPOSE 3000
WORKDIR /app
COPY --from=builder /app/target/release/wikidata-phonemes ./wikidata-phonemes
COPY ./static ./static
ENTRYPOINT ["/app/wikidata-phonemes"]
