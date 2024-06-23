# Dockerfile for building a rust executable that can be run on a specific Ubuntu version (and newer)
FROM ubuntu:20.04 as builder

# Install necessary build tools
RUN apt update && apt -y install curl build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ~/.cargo/bin/rustup update

# Set the working directory in the builder container
WORKDIR /rust/build/

# Copy over your source code
COPY . .

# Build the application in release mode
RUN /bin/bash -c "source ${HOME}/.cargo/env && cargo build --release"

# Start a new stage. This is necessary for smaller image size
FROM ubuntu:20.04

# Add glibc compatibility for Alpine

# Set the working directory in the container to /app
WORKDIR /app

# Copy the binary from the builder stage to the final stage
COPY --from=builder /rust/build/target/release/cuteboard .

# Copy the templates and static files
COPY --from=builder /rust/build/templates ./templates
COPY --from=builder /rust/build/static ./static

# Expose the port the app runs on
EXPOSE 2493

# Define the command to run the app
CMD ["./your_binary_name"]