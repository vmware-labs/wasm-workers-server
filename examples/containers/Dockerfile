# syntax=docker/dockerfile:labs

# Prepare a base layer for building the rust endpoint
FROM --platform=$BUILDPLATFORM rust:1.72.0-alpine AS base
RUN apk add sudo curl musl-dev ca-certificates && \
    curl -fsSL https://workers.wasmlabs.dev/install | sh

# Rust endpoint: first build, then generate the release layer
FROM base AS build-rust
WORKDIR /src
RUN --mount=type=bind,target=/src,source=./apps-src/user-generation-rust \
    cargo build --release --target-dir /output

FROM scratch AS release-rust
COPY --from=build-rust /output/wasm32-wasi/release/user-generation-rust.wasm /
COPY ./apps-src/user-generation-rust/user-generation-rust.toml /

# JS endpoint: no build needed, just generate the release layer
FROM scratch AS release-js
COPY ./apps-src/user-generation-js/ /

# Ruby endpoint: no build needed, just generate the release layer
FROM scratch AS release-ruby
COPY ./apps-src/user-generation-ruby /user-generation-ruby

# Python endpoint: no build needed, just generate the release layer
FROM scratch AS release-python
COPY ./apps-src/user-generation-python/ /user-generation-python

# Go endpoint: first build, then generate the release layer
FROM --platform=$BUILDPLATFORM tinygo/tinygo:0.28.1 AS build-go
WORKDIR /src
RUN --mount=type=bind,target=/src,source=./apps-src/user-generation-go \
    tinygo build \
        -o /home/tinygo/user-generation-go.wasm \
        -no-debug -panic=trap -scheduler=none -gc=leaking \
        -target=wasi .

FROM scratch AS release-go
COPY --from=build-go /home/tinygo/user-generation-go.wasm /
COPY ./apps-src/user-generation-go/user-generation-go.toml /

# Wws root: install the required runtimes, then generate the release layer
FROM base AS build-root
WORKDIR /output
RUN wws runtimes install ruby latest
RUN wws runtimes install python latest
COPY ./apps-src/tmp /output/tmp
RUN mkdir /public

FROM scratch AS release-root
COPY --from=build-root /public /public
COPY --from=build-root /output /

# Merge all the release layers into one
FROM scratch AS release
COPY --from=release-root / /
COPY --from=release-rust / /
COPY --from=release-js / /
COPY --from=release-ruby / /
COPY --from=release-python / /
COPY --from=release-go / /

# Copy over te SSL certificates
COPY --from=base /etc/ssl /etc/ssl

ENTRYPOINT ["/"]
