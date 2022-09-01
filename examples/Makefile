.PHONY: all rust-*

rust-basic:
	cd rust-basic && \
	cargo build --target wasm32-wasi --release && \
	cp target/wasm32-wasi/release/rust-basic.wasm ./basic.wasm

rust-kv:
	cd rust-kv && \
	cargo build --target wasm32-wasi --release && \
	mv target/wasm32-wasi/release/rust-kv.wasm ./kv.wasm

all: rust-basic rust-kv
