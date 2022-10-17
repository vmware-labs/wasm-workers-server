.PHONY: build

build:
	cargo build --release

image-amd64:
	docker build -f image/Dockerfile --build-arg ARCH=amd64 -t amd64/wasm-workers-server:latest .

image-arm64v8:
	docker build -f image/Dockerfile --build-arg ARCH=arm64v8 -t arm64v8/wasm-workers-server:latest .

