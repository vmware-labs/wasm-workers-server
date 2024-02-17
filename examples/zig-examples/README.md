# zig-examples

This example Zig project demonstrates how to use the WWS Zig SDK to compile WWS workers from Zig source code.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* [Zig](https://ziglang.org/download/) `0.12.0`

# Build

All specific build confiugrations are in `build.zig` file.

```sh
zig build
```

# Run

This step assumes that you have `wws` installed on your system.

```sh
wws ./zig-out/root
```

## Resources

* [Zig documentation](https://workers.wasmlabs.dev/docs/languages/zig)
* [Announcing Zig support for Wasm Workers Server](https://wasmlabs.dev/articles/Zig-support-on-wasm-workers-server/)
* [Environment variables](https://workers.wasmlabs.dev/docs/features/environment-variables)
* [Key / Value store](https://workers.wasmlabs.dev/docs/features/key-value)
* [Dynamic routes](https://workers.wasmlabs.dev/docs/features/dynamic-routes)
