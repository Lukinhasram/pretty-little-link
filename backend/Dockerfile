# ---- Builder Stage ----
# Use the official Rust image. The 'slim' variant is smaller.
FROM rust:1-slim as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo files to cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./

# Create a dummy src directory to build dependencies without the full source code
# This is a Docker caching trick to avoid re-downloading dependencies on every code change.
RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/

# Now copy the actual source code
COPY ./src ./src

# Build the application, leveraging the cached dependencies
RUN cargo build --release

# ---- Runner Stage ----
# Use a minimal, secure base image for the final container
FROM debian:bookworm-slim

# Set a non-root user for security best practices
RUN groupadd --system --gid 1001 appgroup && \
    useradd --system --uid 1001 --gid appgroup appuser
USER appuser

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/

# Expose the port the app runs on
EXPOSE 3000

# Set the command to run the application
CMD ["/usr/local/bin/backend"]
