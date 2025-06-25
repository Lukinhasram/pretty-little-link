FROM rust:1-slim as builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./

RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/

COPY ./src ./src

RUN cargo build --release

FROM debian:bookworm-slim

# Set a non-root user for security best practices
RUN groupadd --system --gid 1001 appgroup && \
    useradd --system --uid 1001 --gid appgroup appuser
USER appuser

COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/

EXPOSE 3000

CMD ["/usr/local/bin/backend"]
