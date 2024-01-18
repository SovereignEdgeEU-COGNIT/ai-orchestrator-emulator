# Use an official Rust image as the base
FROM rust:latest

# Install necessary system packages
RUN apt-get update && apt-get install -y \
    stress-ng \
    iproute2 \
    && rm -rf /var/lib/apt/lists/*

# Copy your Rust project into the container
WORKDIR /usr/src/myapp
COPY . .

# Build your Rust application
RUN cargo install --path .

# Expose necessary ports (adjust if your application uses different ports)
# Expose port for your Rust application
EXPOSE 8000

# Add a command to run your application
CMD ["./target/release/emulated_host"]
