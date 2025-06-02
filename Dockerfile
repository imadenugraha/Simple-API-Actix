FROM rust:1.87-slim AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:12-slim

RUN adduser --shell /bin/sh --disabled-password appuser

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install curl --no-install--recommends -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/*

WORKDIR /app

COPY --from=builder /app/target/release/simple_api_actix /app/simple_api_actix

RUN chown -R appuser:appuser /app

EXPOSE 8000

USER appuser

HEALTHCHECK CMD --interval=1m curl --fail http://localhost:8000/up || exit 1

CMD ["./simple_api_actix"]
