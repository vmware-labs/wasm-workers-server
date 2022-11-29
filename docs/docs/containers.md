---
sidebar_position: 3
---

# Running in a container

For convenience we have published a container image that contains Wasm Workers Server. It is available at `projects.registry.vmware.com/wasmlabs/containers/wasm-workers-server:latest`. Any container that runs it will get the `wws` binary, running and:

 - Looking for workers in the `/app` folder
 - Listening on `0.0.0.0:8080` inside the container

The image is based on `debian:bullseye-slim` + the `wws` binary. It includes support for the `linux/amd64` and `linux/arm64/v8` platforms. The image size should be around `~100MiB`

## Running a local container

A typical one-liner to run a local container for development purposes would look like:

```bash
docker run -v /path/to/workers/on/host:/app -p 8080:8080 \
 projects.registry.vmware.com/wasmlabs/containers/wasm-workers-server:latest
```

## Other usages

Wasm Workers Server is stateless as far as the loaded handers are stateless (i.e. when they don't use the [Key / Value store](./features/key-value.md)). This makes the image very useful if you want to setup your own auto-scaling deployment.

