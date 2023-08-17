# Key/Value example

Compile a Zig worker to WebAssembly and run it in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* [Zig](https://ziglang.org/download/) `0.11.0`

## Build

All specific build confiugrations are in `build.zig` file.

```shell-session
zig build
```

## Run

```shell-session
wws ./zig-out/bin/
```

## Resources

* [Key / Value store](https://workers.wasmlabs.dev/docs/features/key-value)
* [Zig documentation](https://workers.wasmlabs.dev/docs/languages/zig)
* [Announcing Zig support for Wasm Workers Server](https://wasmlabs.dev/articles/Zig-support-on-wasm-workers-server/)
