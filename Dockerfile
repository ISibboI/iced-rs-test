FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-opt wasm-pack

WORKDIR /iced-rs-test
COPY Cargo.toml .
COPY Cargo.lock .
COPY .cargo/ .cargo/
RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN cargo build --release --target wasm32-unknown-unknown
RUN rm -rf src

COPY src/ src/
COPY data/ data/
RUN cargo run --offline --release -- --log-level Debug compile
COPY index.html .
RUN trunk build --release

HEALTHCHECK --start-period=5m CMD curl -f localhost:8080 || exit 1
CMD ["trunk", "serve", "--release", "--no-autoreload", "--address", "0.0.0.0"]