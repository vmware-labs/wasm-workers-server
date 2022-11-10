.PHONY: build

build:
	cargo build --release

image-amd64:
	docker build -f image/Dockerfile --platform amd64 -t wasm-workers-server:latest-amd64 .

image-arm64:
	docker build -f image/Dockerfile --platform arm64 -t wasm-workers-server:latest-arm64 .

push-image-multiarch:
	docker buildx build -f image/Dockerfile --platform linux/arm64/v8,linux/amd64 --push -t projects.registry.vmware.com/wasmlabs/containers/wasm-workers-server:latest .
	