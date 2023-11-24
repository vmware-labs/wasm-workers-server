.PHONY: build build-src

build-src:
	cargo build --target wasm32-wasi --release

build: build-src
	wizer --allow-wasi --wasm-bulk-memory true -o ./wasm-workers-quick-js-engine.wasm ./target/wasm32-wasi/release/wasm-workers-quick-js-engine.wasm
