FROM rust:1.94.1-alpine3.20 AS build
WORKDIR /app
COPY Cargo.toml /app
COPY src/ /app/src
RUN apk add --no-cache pkgconf openssl-libs-static openssl-dev \
  && cargo build --release

FROM alpine:3.20 AS runtime
WORKDIR /app
COPY config/ /app/config
COPY --from=build /app/target /app/target
RUN apk add --no-cache openssl-libs-static
ENTRYPOINT [ "/app/target/release/email-rs" ]
