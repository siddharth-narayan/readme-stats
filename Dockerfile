FROM rust:1.97.1-alpine AS builder

WORKDIR /app

RUN apk add musl-dev pkgconfig openssl-dev openssl-libs-static font-noto

COPY . .

RUN cargo build --profile release

RUN mv target/release/readme-stats .
RUN rm -r target/

RUN adduser -D appuser
USER appuser

EXPOSE 3000

CMD ["/app/readme-stats"]