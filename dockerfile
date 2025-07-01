# Étape 1 : build de l'application Rust
FROM rust:1.77 as builder

WORKDIR /app

# Copier tout le code source
COPY . .

# Générer un Cargo.lock (si pas encore fait)
RUN cargo generate-lockfile

# Compiler le projet en release
RUN cargo build --release

# Étape 2 : runtime minimal
FROM debian:buster-slim

# Installer les dépendances système (pour lecture audio)
RUN apt-get update && apt-get install -y libasound2 && rm -rf /var/lib/apt/lists/*

# Copier le binaire compilé et le fichier audio
COPY --from=builder /app/target/release/rust_audio_api /app/app
COPY example.mp3 /app/example.mp3

WORKDIR /app
CMD ["./app"]
