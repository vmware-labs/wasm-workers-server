---
sidebar_position: 2
---

# Quickstart

Wasm Workers runs almost anywhere. Thanks to its portability, downloading and running it anywhere is quite simple.

First, you need to install `wws`:

```bash
curl -fsSL https://workers.wasmlabs.dev/install | bash
```

Now, you can check the different commands and options:

```bash
wws --help

Usage: wws [OPTIONS] [PATH] [COMMAND]

Commands:
  runtimes  Manage the language runtimes in your project
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Folder to read WebAssembly modules from [default: .]

Options:
      --host <HOSTNAME>  Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>      Port to initiate the server [default: 8080]
      --prefix <PREFIX>  Prepend the given path to all URLs [default: ]
  -h, --help             Print help information
  -V, --version          Print version information
```

You can download some of our example `.js` modules:

```bash
curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/examples/js-basic/index.js \
    -o ./index.js
```

Finally, you can run wws and check the response from the worker:

```bash
wws .

⚙️  Loading routes from: ./examples
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.js (name: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

Now, open your browser at <http://127.0.0.1:8080>.

## Next Steps

Now you got the taste of Wasm Workers, it's time to create your first worker:

* [Create your first JavaScript worker](../languages/javascript.md)
* [Create your first Rust worker](../languages/rust.md)
* [Create your first Python worker](../languages/python.md)
* [Create your first Ruby worker](../languages/ruby.md)
* [Create your first Go worker](../languages/go.md)

And if you are curious, here you have a guide about [how it works](./how-it-works.md).