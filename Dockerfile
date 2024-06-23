# Stage 1: Build Stage
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Install Git
RUN apt-get update && apt-get install -y git

# Clone your Git repository
RUN git clone https://github.com/yourusername/your-repo.git .

# Build the Rust project
RUN cargo build --release

# Stage 2: Runtime Stage
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /app/target/release/cuteboard .

# Expose the port your application listens on
EXPOSE 8080

# Command to run the executable
CMD ["./cuteboard"]