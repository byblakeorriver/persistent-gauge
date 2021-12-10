FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 as chef
WORKDIR app

FROM chef as planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN rustup component add rustfmt --toolchain 1.53.0-x86_64-unknown-linux-gnu && \
    cargo install --force cargo-strip && \
    cargo build --release && \
    cargo strip

FROM debian:buster-slim as runtime
WORKDIR app

RUN apt update && \
    apt install -y libssl-dev ca-certificates wget default-libmysqlclient-dev  && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/persistent-gauge .
CMD ["./persistent-gauge"]
