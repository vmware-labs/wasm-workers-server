# Build wasm_runtime in release mode

ARG IMAGE_REPOSITORY=docker.io
ARG ARCH=amd64

FROM $IMAGE_REPOSITORY/$ARCH/rust:1.64.0-slim as build-wws
ARG WWS_BUILD_DIR=/usr/src/wws
ARG ARCH
WORKDIR $WWS_BUILD_DIR
COPY ./ $WWS_BUILD_DIR/
RUN set -eux; \
    ls -l .; \
    case "${ARCH}" in \
        amd64) bldArch='x86_64-unknown-linux-gnu' ;; \
        arm64v8) bldArch='aarch64-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: $ARCH"; exit 1 ;; \
    esac; \
    rustup target add $bldArch; \
    cargo build --release --target=$bldArch; \
    mkdir ./build; \
    cp ./target/$bldArch/release/wws ./build/wws

FROM $IMAGE_REPOSITORY/$ARCH/debian:bullseye-slim
ARG WWS_BUILD_DIR=/usr/src/wws
RUN mkdir -p /app
RUN mkdir -p /opt
COPY --from=build-wws ${WWS_BUILD_DIR}/build/wws /opt

CMD ["/opt/wws", "/app"]

