# Docker + Wasm + Wasm Workers Server (wws)

This repo showcases some functions you can write, taking advantage of
Wasm Workers Server, on top of Docker Desktop, thanks to the
[`containerd-wasm-shims`](https://github.com/deislabs/containerd-wasm-shims) project.

## Build

Prerequisites for building this project:

- Docker, with [Docker + Wasm support](https://docs.docker.com/desktop/wasm/)

In order to build this example, you just have to run on the root of
this project:

```shell-session
$ make build
```

## Running

Prerequisites for running this project: Docker Desktop 4.23.0 or later.

You can run the example:

```shell-session
$ make run
```

After that, you can target the different endpoints exposed by the Wasm
Workers Server:

```shell-session
$ curl -s http://localhost:3000/user-generation-rust | jq
$ curl -s http://localhost:3000/user-generation-go | jq
$ curl -s http://localhost:3000/user-generation-js | jq
$ curl -s http://localhost:3000/user-generation-python | jq
$ curl -s http://localhost:3000/user-generation-ruby | jq
```

This example also showcases exposing a directory in the host to the WebAssembly guest. This example can be executed with:

```shell-session
$ make run-with-mount
```

You can reach the same endpoints, but you will notice that the
attribute `.some_file_contents` of the produced JSON in all examples
now is the content of
[tmp/file.txt](tmp/file.txt)
from the host.

The only worker that is not able to read contents from the filesystem
is the JS one, so you can only check it with the rest:

```shell-session
$ curl -s http://localhost:3000/user-generation-rust | jq
$ curl -s http://localhost:3000/user-generation-go | jq
$ curl -s http://localhost:3000/user-generation-python | jq
$ curl -s http://localhost:3000/user-generation-ruby | jq
```
