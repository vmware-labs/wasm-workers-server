# Zig kit

This folder contains the Zig kit or SDK for Wasm Workers Server. Currently, it uses the regular STDIN / STDOUT approach to receive the request and provide the response.

> *Note: this assumes Zig `0.11.0`*

## Build

To build all examples in ./examples

```shell-session
$ zig build -Dtarget="wasm32-wasi"
```

To build a specific example:

```shell-session
$ zig build-exe examples/<example>.zig -target wasm32-wasi
```

## Testing

At `./kits/zig/worker` execute:

```shell-session
$ zig build -Dtarget="wasm32-wasi"
$ wws ./zig-out/bin/
```

## sockaddr issue

Using `http.Server.Response` was unsuccessful and lead to following error:

```
$ worker git:(144_-_add_support_for_zig) ✗ zig build -Dtarget="wasm32-wasi"
zig build-exe main Debug wasm32-wasi: error: the following command failed with 1 compilation errors:
/Users/c.voigt/.asdf/installs/zig/0.11.0/zig build-exe /Users/c.voigt/go/src/github.com/voigt/wasm-workers-server/kits/zig/worker/examples/main.zig --cache-dir /Users/c.voigt/go/src/github.com/voigt/wasm-workers-server/kits/zig/worker/zig-cache --global-cache-dir /Users/c.voigt/.cache/zig --name main -target wasm32-wasi -mcpu generic --mod worker::/Users/c.voigt/go/src/github.com/voigt/wasm-workers-server/kits/zig/worker/src/worker.zig --deps worker --listen=- 
Build Summary: 6/9 steps succeeded; 1 failed (disable with --summary none)
install transitive failure
└─ install main transitive failure
   └─ zig build-exe main Debug wasm32-wasi 1 errors
/Users/c.voigt/.asdf/installs/zig/0.11.0/lib/std/os.zig:182:28: error: root struct of file 'os.wasi' has no member named 'sockaddr'
pub const sockaddr = system.sockaddr;
                     ~~~~~~^~~~~~~~~
referenced by:
    Address: /Users/c.voigt/.asdf/installs/zig/0.11.0/lib/std/net.zig:18:12
    Address: /Users/c.voigt/.asdf/installs/zig/0.11.0/lib/std/net.zig:17:28
    remaining reference traces hidden; use '-freference-trace' to see all reference traces
```