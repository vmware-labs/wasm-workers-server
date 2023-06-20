# Rust Key / Value example

Compile a Rust worker to WebAssembly and run it in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* [Install Rust with rustup](https://www.rust-lang.org/tools/install)
* Install the `wasm32-wasi` target:

    ```shell-session
    rustup target add wasm32-wasi
    ```

## Build

```shell-session
cargo build --target wasm32-wasi --release && \
	cp target/wasm32-wasi/release/rust-kv.wasm ./kv.wasm
```

## Run

```shell-session
wws .
```

## Resources

* [Key / Value store](https://workers.wasmlabs.dev/docs/features/key-value)
* [Rust documentation](https://workers.wasmlabs.dev/docs/languages/rust)
