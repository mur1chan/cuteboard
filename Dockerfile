# Dockerfile for building a rust executable
FROM rust:1.79-buster

# Set the working directory in the container to /app
WORKDIR /app

# Copy over your source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 2493

# Define the command to run the app
CMD ["./target/release/cuteboard"]