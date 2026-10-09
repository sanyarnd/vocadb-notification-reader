FROM rust:1.99-slim-trixie AS builder
WORKDIR /build

# Cache dependencies separately from the application sources.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && touch src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release --locked

FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=builder /build/target/release/vocadb-notification-reader /usr/local/bin/vocadb-notification-reader

ENV LISTEN_ADDR=0.0.0.0:8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/vocadb-notification-reader"]
