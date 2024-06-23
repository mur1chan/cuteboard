# Use the official Rust image as a builder
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the project
RUN cargo build --release

# Use a newer base image for the final output
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/cuteboard .

# Copy the templates and static files
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

# Expose the port the app runs on
EXPOSE 2493

# Define the command to run the app
CMD ["./cuteboard"]
