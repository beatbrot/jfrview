test:
    cargo test
    wasm-pack test --headless --firefox .

lint:
    cargo clippy

build-profiling:
    wasm-pack build -t web --profiling

[working-directory('js-tests')]
node-bench: build-profiling
    npm ci
    npm run bench
