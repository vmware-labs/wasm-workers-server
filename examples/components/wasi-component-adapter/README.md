# WASI component adapter

The Bytecode Alliance provides a WASI component adapter that allows a
WASI WebAssembly module to be compiled to a WebAssembly component.

You can reproduce this WASI component adapter by running:

```shell-session
$ git clone git@github.com:bytecodealliance/wasmtime.git
$ cd wasmtime
$ cargo build -p wasi-preview1-component-adapter \
    --target wasm32-unknown-unknown --release
```

You can find the WASI component adapter at
`target/wasm32-unknown-unknown/release/wasi_snapshot_preview1.wasm`.

This adapter is valid for a WASI reactor. If you want to build an
adapter for a WASI command, you can run:

```shell-session
$ git clone git@github.com:bytecodealliance/wasmtime.git
$ cd wasmtime
$ cargo build -p wasi-preview1-component-adapter \
    --features command \
    --no-default-features \
    --target wasm32-unknown-unknown --release
```

## Converting a WebAssembly module to a WebAssembly component

In order to convert a WebAssemby module to a WebAssembly component,
you can achieve this goal by using `wasm-tools`, along with the WASI
component adapter you have just built.

You can link to the reactor WASI adapter like so:

```shell-session
$ wasm-tools component new my-wasm-module.wasm \
    --adapt wasi_snapshot_preview1=wasi_snapshot_preview1-reactor.wasm \
    -o my-wasm-component.wasm
```

Or you can link to the command WASI adapter like the following:

```shell-session
$ wasm-tools component new my-wasm-module.wasm \
    --adapt wasi_snapshot_preview1=wasi_snapshot_preview1-command.wasm \
    -o my-wasm-component.wasm
```

In both cases, you can check that the component is valid, and output
the WIT definition associated with it, like:

```shell-session
$ wasm-tools validate my-wasm-component.wasm --features component-model
$ wasm-tools component wit component.wasm
```
