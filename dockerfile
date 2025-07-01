# Étape 1 : build
FROM rust:1.77 AS builder

WORKDIR /app
COPY . .

# Précharger les dépendances
RUN cargo fetch

# Compiler le projet
RUN cargo build --release

# Étape 2 : image finale
FROM debian:bullseye-slim

# Installer les bibliothèques audio nécessaires
RUN apt-get update && apt-get install -y libasound2 && rm -rf /var/lib/apt/lists/*

# Copier l'exécutable
COPY --from=builder /app/target/release/rust_audio_api /app/app
WORKDIR /app

EXPOSE 8080
CMD ["./app"]
