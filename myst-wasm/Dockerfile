# cannot produce proc-macro for `pin-project-internal v0.4.16` as the target `x86_64-unknown-linux-musl` does not support these crate types
# FROM rust:alpine as builder
FROM rust:1.65-slim as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /app/myst-wasm

COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY index.scss index.scss
COPY index.html index.html
RUN trunk build --release

# copy the built outputs into a much lighter nginx image
FROM nginx
COPY --from=builder /app/myst-wasm/dist /usr/share/nginx/html

# need to add the application/wasm mime type to NGINX - it doesn't come with it by default!
RUN sed -i '97i application/wasm wasm;' /etc/nginx/mime.types

EXPOSE 80
ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]