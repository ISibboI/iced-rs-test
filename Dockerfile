FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-opt wasm-pack

WORKDIR /iced-rs-test
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir -p src
RUN touch src/main.rs
RUN cargo fetch
RUN rm -rf src

COPY src/ src/
COPY index.html .
RUN trunk build --release

CMD ["trunk", "serve", "--release", "--no-autoreload", "--address", "0.0.0.0"]