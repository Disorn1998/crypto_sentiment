# Stage 1: Build (ใช้ Rust เวอร์ชันล่าสุด 1.x)
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# ติดตั้ง dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy โค้ด
COPY . .

# Build
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim

WORKDIR /app

# ติดตั้ง CA Certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy ไฟล์โปรแกรมที่ Build เสร็จแล้ว
COPY --from=builder /app/target/release/worker /app/worker

# รันโปรแกรม
CMD ["./worker"]