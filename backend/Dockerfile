# --- Stage 1: Build ---
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# --- Stage 2: Production ---
FROM debian:buster-slim

# CORREGIDO: El binario se llama "backend" según Cargo.toml
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

EXPOSE 8080

# CORREGIDO: Ejecutar "backend", no "maya-digital-platform" 
CMD ["backend"]
