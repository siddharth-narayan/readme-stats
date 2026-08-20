FROM rust:1.97.1-alpine AS builder

WORKDIR /app

RUN apk add musl-dev pkgconfig openssl-dev openssl-libs-static

COPY . .

RUN cargo build --profile release

RUN mv target/release/readme-stats .
RUN rm -r target/


FROM alpine:3.24

WORKDIR /app

RUN apk add --no-cache font-noto-all font-noto-cjk font-noto-emoji
RUN adduser -D appuser

COPY --from=builder /app /app

USER appuser
EXPOSE 3000
CMD ["/app/readme-stats"]