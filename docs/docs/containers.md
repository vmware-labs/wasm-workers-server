---
sidebar_position: 5
---

# Running in a container

For convenience we have published a container image that contains Wasm Workers Server. It is available at `ghcr.io/vmware-labs/wws:latest`. Any container that runs it will get the `wws` binary, running and:

 - Looking for workers in the `/app` folder
 - Listening on `0.0.0.0:8080` inside the container

The image is built from `scratch`. It only includes the `wws` binary. The container supports multiple architectures: `linux/amd64` and `linux/arm64` platforms. The image size is just `27MiB`.

## Running a local container

A typical one-liner to run a local container for development purposes would look like:

```bash
docker run -v /path/to/workers/on/host:/app -p 8080:8080 \
 ghcr.io/vmware-labs/wws:latest
```

## Other usages

Wasm Workers Server is stateless as far as the loaded handers are stateless (i.e. when they don't use the [Key / Value store](./features/key-value.md)). This makes the image very useful if you want to setup your own auto-scaling deployment.

