FROM rust:1.91 as builder
WORKDIR /app
# Copy only dependency files first
COPY Cargo.toml Cargo.lock ./
# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src
# Now copy actual source code
COPY src ./src
# Build with cached dependencies
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/book-store .
COPY images ./images
COPY mockup ./mockup
COPY .env.example .
EXPOSE 3000
CMD ["./book-store"]