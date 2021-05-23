# Builder stage
FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM alpine:latest AS runtime

WORKDIR /app
# Copy the compiled binary from the builder environment 
# to our runtime environment
COPY --from=builder /app/target/release/bunnylol bunnylol
# We need the configuration file at runtime!
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
ENTRYPOINT ["./bunnylol"]