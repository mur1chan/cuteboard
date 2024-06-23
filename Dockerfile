# Use a base image with necessary dependencies
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty dummy project to download dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy the rest of the application source code
COPY . .

# Build the application
RUN cargo build --release

# Final stage: Use a smaller base image to reduce the image size
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage to the final image
COPY --from=builder /app/target/release/myapp /app/myapp

# Expose any necessary ports
EXPOSE 8080

# Command to run the application
CMD ["./myapp"]