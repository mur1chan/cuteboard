# Use the official Rust image with musl for aarch64 as a builder
FROM messense/rust-musl-cross:aarch64-musl as builder

# Set the working directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the project for aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

# Use a smaller base image for the final output
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/cureboard .

# Copy the templates and static files
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

# Expose the port the app runs on
EXPOSE 2493

# Define the command to run the app
CMD ["./cuteboard"]
