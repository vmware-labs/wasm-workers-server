# Workaround for https://github.com/moby/buildkit/issues/3891
export BUILDX_NO_DEFAULT_ATTESTATIONS = 1

# Build a container image for the demo
.PHONY: image
image:
	docker build --platform wasi/wasm --tag=ghcr.io/vmware-labs/wws-apps:latest .

# Export the content of the demo image into the ./dist folder
.PHONY: dist
dist: clean
	docker build --platform wasi/wasm --output=dist .

# Run the demo container
.PHONY: run
run: stop image
	docker run --rm -d --name docker-wws \
	  -p 3000:3000 \
	  --runtime=io.containerd.wws.v1 \
	  --platform=wasi/wasm \
	 ghcr.io/vmware-labs/wws-apps:latest
	@echo "Now you can reach the Wasm Workers Server functions, such as:"
	@echo "  - curl http://localhost:3000/user-generation-rust"
	@echo "  - curl http://localhost:3000/user-generation-go"
	@echo "  - curl http://localhost:3000/user-generation-js"
	@echo "  - curl http://localhost:3000/user-generation-python"
	@echo "  - curl http://localhost:3000/user-generation-ruby"

# Run the demo container using a host mount
.PHONY: run-with-mount
run-with-mount: stop image
	docker run --rm -d --name docker-wws \
	  -p 3000:3000 \
	  --runtime=io.containerd.wws.v1 \
	  --platform=wasi/wasm \
	  -v $(PWD)/tmp:/tmp \
	 ghcr.io/vmware-labs/wws-apps:latest
	@echo "Now you can reach the Wasm Workers Server functions, such as:"
	@echo "  - curl http://localhost:3000/user-generation-rust"
	@echo "  - curl http://localhost:3000/user-generation-go"
	@echo "  - curl http://localhost:3000/user-generation-js"
	@echo "  - curl http://localhost:3000/user-generation-python"
	@echo "  - curl http://localhost:3000/user-generation-ruby"

# Stop the demo contianer
.PHONY: stop
stop:
	docker rm -f docker-wws

# Same as dist
.PHONY: build
build: dist;

.PHONY: clean
clean:
	rm -Rf ./dist
