FROM rust:1.77 as builder
WORKDIR /app

COPY . .

# ⚠️ Ajoute cette ligne : génère un lockfile compatible dans Docker
RUN cargo generate-lockfile

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libasound2 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_audio_api /app/app
COPY example.mp3 /app/example.mp3
WORKDIR /app
EXPOSE 8080
CMD ["./app"]
