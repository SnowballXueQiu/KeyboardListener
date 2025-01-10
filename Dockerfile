FROM rust AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:12-slim
WORKDIR /app
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/kbl-backend .
CMD ["./kbl-backend"]