# Use an official Rust image as the base
FROM rust:latest

# Copy your Rust project into the container
WORKDIR /usr/src/myapp
COPY . .

# Build your Rust application
RUN cargo install --path ./ctrl_plane_server

RUN rm -rf ./target

# Expose necessary ports (adjust if your application uses different ports)
# Expose port for your Rust application
EXPOSE 8000

# Add a command to run your application
CMD ["server"]
