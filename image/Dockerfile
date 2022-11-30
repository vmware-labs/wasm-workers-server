# Build Wasm Workers in release mode. This is a multiplatform
# container, so we ensure we build it for amd64 and arm64.
# This will require the Rust configuration to change when
# building the project
FROM --platform=$TARGETPLATFORM rust:1.65.0-slim as build-wws
ARG WWS_BUILD_DIR=/usr/src/wws
ARG TARGETPLATFORM
ARG BUILDPLATFORM
WORKDIR $WWS_BUILD_DIR
# Only copy required files
COPY ./Cargo.* $WWS_BUILD_DIR/
COPY ./src $WWS_BUILD_DIR/src
COPY ./kits $WWS_BUILD_DIR/kits
RUN echo "Running on ${BUILDPLATFORM}, building for ${TARGETPLATFORM}"

# Switch to nightly to avoid memory issues when pulling Crates.io
# https://github.com/rust-lang/cargo/issues/10781
RUN apt-get update && apt-get install -y musl musl-dev musl-tools
RUN set -eux; \
    ls -l .; \
    case "${TARGETPLATFORM}" in \
        linux/amd64) bldArch='x86_64-unknown-linux-musl' ;; \
        linux/arm64) bldArch='aarch64-unknown-linux-musl' ;; \
        *) echo >&2 "unsupported architecture: $BUILDPLATFORM"; exit 1 ;; \
    esac; \
    rustup default nightly; \
    rustup target add $bldArch; \
    cargo +nightly build --release --target=$bldArch -Z sparse-registry; \
    mkdir ./build; \
    cp ./target/$bldArch/release/wws ./build/wws

# Build the final image
FROM --platform=$TARGETPLATFORM scratch
ARG WWS_BUILD_DIR=/usr/src/wws
LABEL org.opencontainers.image.source=https://github.com/vmware-labs/wasm-workers-server
LABEL org.opencontainers.image.description="Wasm Workers Server is a blazing-fast self-contained server that routes HTTP requests to workers in your filesystem. Everything run in a WebAssembly sandbox."
LABEL org.opencontainers.image.licenses="Apache-2.0"
COPY --from=build-wws ${WWS_BUILD_DIR}/build/wws /

CMD ["/wws", "/app/", "--host", "0.0.0.0"]

