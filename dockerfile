# Étape 1 : build
FROM rust:1.77 as builder
WORKDIR /app

COPY . .
RUN cargo build --release

# Étape 2 : image finale légère
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libasound2 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_audio_api /app/app
WORKDIR /app
EXPOSE 8080
CMD ["./app"]
