FROM rust:slim AS builder

ENV TARGET x86_64-unknown-linux-musl
ENV PKG_CONFIG_SYSROOT_DIR /
RUN rustup target add "$TARGET"


RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl sqlite3 libsqlite3-dev libssl-dev pkg-config

WORKDIR /app
COPY . .
RUN cargo build --release --locked --target "$TARGET"

# and then copy it to an empty docker image
FROM scratch

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/myst-batch /usr/local/bin

CMD ["myst-batch"]
