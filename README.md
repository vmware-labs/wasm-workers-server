# Wasm Workers Server

Wasm Workers Server (`wws`) is an HTTP server that runs applications with WebAssembly. These applications are composed by multiple modules, called "handlers" or "functions". Each of these modules is in charge of replying to a specific HTTP path in your application.

The server loads the existing Wasm modules and compatible languages in the current path. The filenames and folders determine the final routes that will be served. This is called "filesystem routing" and is a popular technique. Successful frameworks such as [NextJS](https://nextjs.org/) and [Eleventy](https://www.11ty.dev/) work in this way.

Here you have an example of running the tool:

```bash
$ ls .
index.wasm api

$ wws .
⚙️  Loading routes from: .
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.wasm (handler: default)
    - http://127.0.0.1:8080/api/hello
      => api/hello.js (handler: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

## Getting Started

Wasm Workers Server runs almost anywhere. Thanks to its portability, downloading and running it anywhere is quite simple:

```bash
curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/install.sh | bash && \
  wws --help
```

You will see the help of the server:

```
Usage: wws [OPTIONS] [PATH]

Arguments:
  [PATH]  Folder to read WebAssembly modules from [default: .]

Options:
      --host <HOSTNAME>  Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>      Port to initiate the server [default: 8080]
  -h, --help             Print help information
  -V, --version          Print version information
```

Then, you can download some of our example modules and try them directly:

```bash
curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/examples/js-basic/handler.js \
  -o ./index.js && \
  wws .
```

The server will start immediately:

```
⚙️  Loading routes from: .
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.js (handler: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

Now, open your browser at <http://127.0.0.1:8080>.

### Local installation

By default, our `install.sh` script will place the `wws` binary in the `/usr/local/bin` path. If you want to install it in your current path, you can run the installer with the `--local` option:

```bash
curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/install.sh | bash -s -- --local && \
  ./wws --help
```

## Language Support

Wasm Workers Server focuses on simplicity. We want you to run handlers (written in different languages) safely in WebAssembly. For interpreted languages, we add different interpreters:

| Language | Wasm module | Interpreter |
| --- | --- | --- |
| Rust | ✅ | ❌ |
| JavaScript | ❌ | ✅ |
| ... | ... | ... |

We will include more interpreters in the future.

### JavaScript handlers

The integrated interpreter is based on [QuickJS](https://bellard.org/quickjs/) (compiled with the [quickjs-wasm-rs](https://crates.io/crates/quickjs-wasm-rs) crate). The compatible handlers follow the Web Workers API approach. However, not all the Web Workers API is available in these handlers. These are some of the missing features:

- No modules available. Handlers must be a single file
- Fetch API
- Async / Await

We will work on including these features in the future.

## Development

### Prerequisites

To work with this project you will need to install:

* [Rust](https://www.rust-lang.org/tools/install)
* Make

## Run the project

After installing the different prerequisites, you can run the development environment with:

```
$ cargo run -- --help
```

This command will run the server and look for `.wasm` and compatible modules (like `.js`) in the folder you pass via arguments. Check the [examples](./examples/) folder to get more information about creating Wasm handlers.

### Documentation

* `src`: includes the source code for the Wasm Workers Server project
* `examples`: folder to generate different example handlers. Check the README file inside to get more information about how to build those
