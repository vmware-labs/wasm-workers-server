# Go Key / Value store example

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
tinygo build -o counter.wasm -target wasi counter.go
```

## Run

```shell-session
wws .
```

## Resources

* [Key / Value store](https://workers.wasmlabs.dev/docs/features/key-value)
* [Go documentation](https://workers.wasmlabs.dev/docs/languages/go)
* [Announcing Go support for Wasm Workers Server](https://wasmlabs.dev/articles/go-support-on-wasm-workers-server/)
