FROM rust:latest AS builder

WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml ./

# Create a dummy src to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/ems_api /app/ems_api

# Create uploads directory
RUN mkdir -p uploads/payslips

# Expose port
EXPOSE 8000

# Run the application
CMD ["./ems_api"]

