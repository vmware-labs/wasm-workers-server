# Wasm Workers Server

Wasm Workers Server (`wws`) is a framework to develop and run serverless applications server in WebAssembly. These applications are composed by multiple modules called "workers". Each of these tiny modules is in charge of replying to a specific HTTP path in your application.

The server loads the existing Wasm modules and compatible languages in the given path. The filenames and folders determine the final routes that will be served. This is called "filesystem routing" and is a popular technique. Successful frameworks such as [NextJS](https://nextjs.org/) and [Eleventy](https://www.11ty.dev/) work in this way.

Here you have an example of running the tool:

```bash
$ ls .
index.wasm api

$ wws .
⚙️  Loading routes from: .
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.wasm (name: default)
    - http://127.0.0.1:8080/api/hello
      => api/hello.js (name: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

## Documentation

All our documentation is available at <https://workers.wasmlabs.dev>.

## Get Started

Wasm Workers Server runs almost anywhere. Thanks to its portability, downloading and running it anywhere is quite simple:

```bash
curl -fsSL https://workers.wasmlabs.dev/install | bash && \
  wws --help
```

You will see the help of the server:

```
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
  -h, --help             Print help
  -V, --version          Print version
```

Then, you can download some of our example modules and try them directly:

```bash
curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/examples/js-basic/index.js \
  -o ./index.js && \
  wws .
```

The server will start immediately:

```
⚙️  Loading routes from: .
🗺  Detected routes:
    - http://127.0.0.1:8080/
      => index.js (name: default)
🚀 Start serving requests at http://127.0.0.1:8080
```

Now, open your browser at <http://127.0.0.1:8080>.

### Local installation

By default, our `install.sh` script will place the `wws` binary in the `/usr/local/bin` path. If you want to install it in your current path, you can run the installer with the `--local` option:

```bash
curl -fsSL https://workers.wasmlabs.dev/install | bash -s -- --local && \
  ./wws --help
```

### Running in a container

If you don't want to install anything locally you can just run `wws` from the `ghcr.io/vmware-labs/wws:latest` container image. All you need to do is:

 - Map a local folder with workers to `/app` within the container
 - Expose port `8080` from the container

Here is how to quickly run a container with an ad-hoc worker from the `/tmp/wws-app` folder:

```bash
mkdir /tmp/wws-app 2>/dev/null;
echo 'addEventListener("fetch", (e) => { return e.respondWith(new Response("Hello from WWS\n"));});' > /tmp/wws-app/index.js;
docker run --rm -v /tmp/wws-app:/app -p 8080:8080 ghcr.io/vmware-labs/wws:latest
```
## Language Support

Wasm Workers Server focuses on simplicity. We want you to run workers (written in different languages) safely in WebAssembly. For interpreted languages, we add different interpreters:

| Language | Support | Requires an external runtime |
| --- | --- | --- |
| Rust | ✅ | No |
| JavaScript | ✅ | No |
| Go | ✅ | No |
| Ruby | ✅ | [Yes](https://workers.wasmlabs.dev/docs/languages/ruby#installation) |
| Python | ✅ | [Yes](https://workers.wasmlabs.dev/docs/languages/python#installation) |
| ... | ... | ... |

To get more information about multi-language support in Wasm Workers Server, [check our documentation](https://workers.wasmlabs.dev/docs/languages/introduction).

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

This command will run the server and look for `.wasm` and compatible modules (like `.js`) in the folder you pass via arguments. Check the [examples](./examples/) folder to get more information about creating Wasm workers.

### Documentation

* `src`: includes the source code for the Wasm Workers Server project
* `examples`: folder to generate different example workers. Check the README file inside to get more information about how to build those
