test:
    cargo test
    wasm-pack test --headless --firefox .

lint:
    cargo clippy
