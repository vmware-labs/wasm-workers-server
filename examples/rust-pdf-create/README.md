# Rust PDF example

Compile a Rust worker to WebAssembly and run it in Wasm Workers Server. This worker generates a PDF based on the request body.

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
	cp target/wasm32-wasi/release/rust-pdf-create.wasm ./index.wasm
```

## Run

```shell-session
wws .
```

## Resources

* [Rust documentation](https://workers.wasmlabs.dev/docs/languages/rust)
