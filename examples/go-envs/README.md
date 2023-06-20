# Go environment variables example

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
tinygo build -o envs.wasm -target wasi envs.go
```

## Run

```shell-session
wws .
```

## Resources

* [Environment variables](https://workers.wasmlabs.dev/docs/features/environment-variables)
* [Go documentation](https://workers.wasmlabs.dev/docs/languages/go)
* [Announcing Go support for Wasm Workers Server](https://wasmlabs.dev/articles/go-support-on-wasm-workers-server/)
