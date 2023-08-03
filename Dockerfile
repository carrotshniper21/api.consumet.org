# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/consumet-api

# Copy the Rust application's source code into the container
COPY . .

RUN rustup default nightly

ENV PORT=8080

# Build the Rust application inside the container
RUN cargo build --release

# Expose any necessary ports (if your app requires it)
EXPOSE 8080

# Specify the command to run your application (modify as needed)
CMD ["./target/release/consumet-api"]
