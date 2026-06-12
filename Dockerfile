FROM rust:1.95.0-alpine3.23 AS builder

RUN apk update && \
    apk add --no-cache bash curl libc-dev binaryen

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.6/cargo-leptos-installer.sh | sh

RUN rustup target add wasm32-unknown-unknown

WORKDIR /work

COPY app ./app
COPY server ./server
COPY public ./public
COPY style ./style
COPY Cargo.toml ./
COPY Cargo.lock ./

RUN cargo leptos build --release -vv

### Additional compression with UPX
FROM alpine:3.19 AS compressor
RUN apk add --no-cache upx

COPY --from=builder /work/target/release/server /server
RUN upx --best --lzma /server

#FROM alpine:3.22.4 AS runner
FROM scratch AS runner

WORKDIR /app

COPY --from=builder /work/site /app/site
COPY --from=compressor /server /app/
COPY --from=builder /work/Cargo.toml /app/
COPY --from=builder /work/server/migrations /app/

EXPOSE 8080
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/server"]
