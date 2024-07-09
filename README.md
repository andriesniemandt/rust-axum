# Rust Axum

## Install required dependencies

```bash
cargo add tokio --features full && \
cargo add serde@1.0.195 --features derive && \
cargo add chrono@0.4.34 --features serde && \
cargo add axum@0.7.5 jsonwebtoken@9.3.0 bcrypt@0.15.1 serde_json@1.0.95
```

## Compile the project

```bash
cargo build
```

## Run the binary

```bash
./target/debug/rust-axum
```