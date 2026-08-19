FROM rust:1.97.1-alpine AS builder

WORKDIR /app

RUN apk add musl-dev pkgconfig openssl-dev openssl-libs-static

COPY . .

RUN cargo build --profile release

RUN rm -r target/release/build

RUN adduser -D appuser
USER appuser

EXPOSE 3000

CMD ["/app/target/release/readme-stats"]