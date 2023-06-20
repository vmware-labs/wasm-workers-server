---
sidebar_position: 4
---

# How it works?

Wasm Workers is built around two main ideas:

* **Workers receive requests and return responses**.

    We follow this approach as it's a widely used pattern for creating serverless functions. Following this pattern helps us to keep compatibility with multiple platforms and avoid vendor-locking on our own tool.

* **Workers receive and return data via WASI Stdio**.

    To increase compatibility and simplify the integration with existing languages, we decided to send and receive data using `STDIN` / `STDOUT`. So, any language that can be compiled using WASI standard can create compatible workers with Wasm Workers.

## Runner

Based on these two principles, the server performs the following tasks:

* Identify `.wasm` modules and any other supported languages (like `.js` and `.py`) in the given folder.
* Associate a HTTP route to every module.
* Create a Key / Value in-memory store if required.
* Initialize the [Wasmtime](https://wasmtime.dev/) runtime..
* Start a HTTP server to start serving the requests.

## Convention over configuration

Wasm Workers assume the HTTP routes from the filesystem. This approach is pretty similar to other very successful projects like NextJS. This simplifies the server interface by running without adding any configuring file.

For extra features such as the Key / Value store, you need to write a configuration file. By default, Wasm Workers doesn't enable any extra feature to any worker. This is an example configuration file to enable the Key / Value store for a worker:

```toml title="./counter.toml"
name = "counter"
version = "1"

[data]
[data.kv]
namespace = "counter"
```

These files are only required to enable extra features for your workers.
