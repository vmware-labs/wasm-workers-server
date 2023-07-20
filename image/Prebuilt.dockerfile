# In this case, the binaries should be already created. This Dockerfile
# is mainly used to build the preview / release container images in
# GitHub actions

# Retrieve the certificates to install runtimes later on.
FROM --platform=$TARGETPLATFORM bitnami/minideb:latest AS sysroot
RUN mkdir -p /target/app /target/opt
RUN install_packages ca-certificates

# Build the final image
FROM --platform=$TARGETPLATFORM scratch
ARG TARGETPLATFORM
ARG TARGETARCH
LABEL org.opencontainers.image.source=https://github.com/vmware-labs/wasm-workers-server
LABEL org.opencontainers.image.description="Wasm Workers Server is a blazing-fast self-contained server that routes HTTP requests to workers in your filesystem. Everything run in a WebAssembly sandbox."
LABEL org.opencontainers.image.licenses="Apache-2.0"

COPY --from=sysroot /target/app /app
COPY --from=sysroot /target/opt /opt
COPY --from=sysroot /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --chmod=755 ./wws-$TARGETARCH /opt/wws

ENTRYPOINT ["/opt/wws"]
CMD ["/app/", "--host", "0.0.0.0"]
