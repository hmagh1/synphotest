# Étape 1 : build
FROM rust:1.77 AS builder

WORKDIR /app
COPY . .

# Précaution : assure que le lockfile existe
RUN cargo fetch
RUN cargo build --release

# Étape 2 : image légère
FROM debian:bullseye-slim

# Install lib audio si besoin
RUN apt-get update && apt-get install -y libasound2 && rm -rf /var/lib/apt/lists/*

# Copie binaire depuis l’étape 1
COPY --from=builder /app/target/release/rust_audio_api /app/app
WORKDIR /app

EXPOSE 8080
CMD ["./app"]
