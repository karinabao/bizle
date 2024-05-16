# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies only
RUN cargo build --release && rm -r src

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a smaller base image for the final build
FROM debian:buster-slim

# Copy the compiled binary from the build stage
COPY --from=0 /app/target/release/your_app_name /usr/local/bin/your_app_name

# Expose the port Rocket will serve on
EXPOSE 8000

# Set the startup command to run the binary
CMD ["your_app_name"]
