# Build wasm_runtime in release mode


FROM --platform=$TARGETPLATFORM rust:1.64.0-slim as build-wws
ARG WWS_BUILD_DIR=/usr/src/wws
ARG TARGETPLATFORM
ARG BUILDPLATFORM
WORKDIR $WWS_BUILD_DIR
COPY ./ $WWS_BUILD_DIR/
RUN echo "Running on ${BUILDPLATFORM}, building for ${TARGETPLATFORM}"
RUN set -eux; \
    ls -l .; \
    case "${TARGETPLATFORM}" in \
        linux/amd64) bldArch='x86_64-unknown-linux-gnu' ;; \
        linux/arm64) bldArch='aarch64-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: $BUILDPLATFORM"; exit 1 ;; \
    esac; \
    rustup target add $bldArch; \
    cargo build --release --target=$bldArch; \
    mkdir ./build; \
    cp ./target/$bldArch/release/wws ./build/wws


FROM --platform=$TARGETPLATFORM debian:bullseye-slim
ARG WWS_BUILD_DIR=/usr/src/wws
RUN mkdir -p /app
RUN mkdir -p /opt
COPY --from=build-wws ${WWS_BUILD_DIR}/build/wws /opt

CMD ["/opt/wws", "/app/", "--host", "0.0.0.0"]

