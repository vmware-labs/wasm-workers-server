---
sidebar_position: 2
---

# Getting Started

Wasm Workers runs almost anywhere. Thanks to its portability, downloading and running it anywhere is quite simple:

```bash
$ curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/install.sh | bash
$ wws --help
Usage: wws [OPTIONS] [PATH]

Arguments:
  [PATH]  Folder to read WebAssembly modules from [default: .]

Options:
      --host <HOSTNAME>  Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>      Port to initiate the server [default: 8080]
  -h, --help             Print help information
  -V, --version          Print version information
```

You can download some of our example `.wasm` modules and try them:

```bash
$ curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/examples/compiled/hello.wasm \
    -o ./index.wasm
$ wasm-workers .
⚙️  Loading routes from: ./examples
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.wasm (handler: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

Now, open your browser at <http://127.0.0.1:8080>.

## Next Steps

Now you got the taste of Wasm Workers, it's time to create your first handler:

* [Create your first JavaScript handler](./tutorials/javascript-workers.md)
* [Create your first Rust handler](./tutorials/rust-workers.md)

And if you are curious, here you have a guide about [how it works](./how-it-works.md).