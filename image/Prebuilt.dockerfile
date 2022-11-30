# In this case, the binaries should be already created. This Dockerfile
# is mainly used to build the preview / release container images in
# GitHub actions

# Build the final image
FROM --platform=$TARGETPLATFORM scratch
LABEL org.opencontainers.image.source=https://github.com/vmware-labs/wasm-workers-server
LABEL org.opencontainers.image.description="Wasm Workers Server is a blazing-fast self-contained server that routes HTTP requests to workers in your filesystem. Everything run in a WebAssembly sandbox."
LABEL org.opencontainers.image.licenses="Apache-2.0"

COPY ./wws-$TARGETARCH /

CMD ["/wws", "/app/", "--host", "0.0.0.0"]

