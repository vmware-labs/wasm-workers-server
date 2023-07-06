.PHONY: all rust-* pdf-create

rust-basic:
	cd rust-basic && \
	cargo build --target wasm32-wasi --release && \
	cp target/wasm32-wasi/release/rust-basic.wasm ./basic.wasm

rust-fetch:
	cd rust-fetch && \
	cargo build --target wasm32-wasi --release && \
	cp target/wasm32-wasi/release/rust-fetch.wasm ./index.wasm

rust-kv:
	cd rust-kv && \
	cargo build --target wasm32-wasi --release && \
	mv target/wasm32-wasi/release/rust-kv.wasm ./kv.wasm

rust-params:
	cd rust-params && \
	cargo build --target wasm32-wasi --release && \
	mv target/wasm32-wasi/release/rust-params.wasm "./[id].wasm"

rust-pdf-create:
	cd rust-pdf-create && \
	cargo build --target wasm32-wasi --release && \
	mv target/wasm32-wasi/release/rust-pdf-create.wasm ./index.wasm

all: rust-basic rust-fetch rust-kv rust-params
