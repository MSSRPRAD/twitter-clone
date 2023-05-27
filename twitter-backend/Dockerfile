# 
# Build
# 

# Use a minimal Rust installation
FROM rust:1-slim-bullseye AS builder

# Set Work Directory inside container
WORKDIR /code

# Use root inside container
RUN USER=root 

# Update stuff
RUN apt-get update && apt-get install -y \
build-essential checkinstall zlib1g-dev -y

# Create a cargo project inside /code
RUN cargo init

# Copy our Cargo.toml into container
COPY Cargo.toml Cargo.toml

# Fetch the packages
RUN cargo fetch

# Copy src/*
COPY src src

# Build our project inside container
RUN cargo build --release

# 
# Run
# 

# Base image for run stage
FROM debian:bullseye-slim

# Work directory for run
WORKDIR /app

# Copy the compiled binary into work directory
COPY --from=builder /code/target/release/twitter_backend twitter_backend

# For security reasons
USER 1001

# Expose port
EXPOSE 8080

# Run the server
CMD [ "/app/twitter_backend" ]