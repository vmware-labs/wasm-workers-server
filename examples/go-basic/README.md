# Go basic example

Compile a Go worker to WebAssembly and run it in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* [Go](https://go.dev/)
* [TinyGo](https://tinygo.org/getting-started/install/)

## Build

```shell-session
tinygo build -o index.wasm -target wasi main.go
```

## Run

```shell-session
wws .
```

## Resources

* [Go documentation](https://workers.wasmlabs.dev/docs/languages/go)
* [Announcing Go support for Wasm Workers Server](https://wasmlabs.dev/articles/go-support-on-wasm-workers-server/)
