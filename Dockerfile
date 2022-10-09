ARG OSM_FILE_PATH
ARG TARGET=x86_64-unknown-linux-musl
ARG GEOCODING_DIR=/var/reverse-geocoding-rust
ARG BUILD_DIR=$GEOCODING_DIR/target/$TARGET/release

FROM rust:1.64.0 AS builder

ARG TARGET
ARG GEOCODING_DIR
ARG REPOSITORY_GIT_URL=https://github.com/Amraneze/reverse-geocoding-rust

WORKDIR /var/

RUN git clone $REPOSITORY_GIT_URL
RUN rustup target add $TARGET

WORKDIR $GEOCODING_DIR

RUN cargo test --release --target=$TARGET
RUN cargo build --release --target=$TARGET


# Final Stage
FROM alpine:latest

ARG TARGET
ARG BUILD_DIR
ARG OSM_FILE_PATH
ARG GEOCODING_DIR

ENV OSM_FILE_PATH=$OSM_FILE_PATH

WORKDIR /var/

COPY --from=builder $BUILD_DIR/reverse-geocoding .

CMD ./reverse-geocoding -f $OSM_FILE_PATH