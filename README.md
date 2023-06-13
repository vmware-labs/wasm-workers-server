<h1 align="center">Wasm Workers Server</h1>

<p align="center">
  Develop and run serverless applications on WebAssembly 🚀
</p>

<p align="center">
  <a href="https://github.com/vmware-labs/wasm-workers-server/actions/workflows/build.yml">
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/vmware-labs/wasm-workers-server/build.yml?label=Build&style=flat-square">
  </a>
  <a href="https://github.com/vmware-labs/wasm-workers-server/releases/latest">
    <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/vmware-labs/wasm-workers-server?label=Release&style=flat-square">
  </a>
  <a href="https://github.com/vmware-labs/wasm-workers-server/blob/main/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/vmware-labs/wasm-workers-server?color=427ece&label=License&style=flat-square">
  </a>
  <a href="https://github.com/vmware-labs/wasm-workers-server/graphs/contributors">
    <img alt="GitHub contributors" src="https://img.shields.io/github/contributors/vmware-labs/wasm-workers-server?label=Contributors&style=flat-square">
  </a>
</p>

<p align="center">
  <a align="center" href="https://workers.wasmlabs.dev/">workers.wasmlabs.dev</a>
</p>

<br/>

Wasm Workers Server (`wws`) is an open source tool to **develop and run serverless applications server on top of WebAssembly**. The applications are composed by multiple modules called [_workers_](https://workers.wasmlabs.dev/docs/get-started/introduction#whats-a-worker). Each of these tiny modules is in charge of replying to a specific HTTP endpoint in your application.

When you start `wws`, it loads the existing workers in the given path or remote repository. You can write a worker in [different languages](https://workers.wasmlabs.dev/docs/languages/introduction) like Rust, JavaScript, Go, Ruby and Python. The filenames and folders determine the final routes that will be served. For example, the `index.js` will reply to the `/` endpoint.

## Getting started (5 minutes)

1. Download and install Wasm Worker Sever:

   ```
   curl -fsSL https://workers.wasmlabs.dev/install | bash
   ```

2. Create an `index.js` file with the following content:

    ```javascript
    addEventListener("fetch", event => {
      return event.respondWith(
        new Response("Hello from Wasm Workers Server!")
      );
    });
    ```

3. Run `wws`:

    ```bash
    $ wws .
    ⚙️  Preparing the project from: .
    ⚙️  Loading routes from: .
    ⏳ Loading workers from 1 routes...
    ✅ Workers loaded in 141.613666ms.
        - http://127.0.0.1:8080/
          => ./index.js
    🚀 Start serving requests at http://127.0.0.1:8080
    ```

4. Access to <http://127.0.0.1:8080>.

Congrats! You just created your first worker 🎉. You can now try with your favorite language:

* [Create your first Rust worker](https://workers.wasmlabs.dev/docs/languages/rust)
* [Create your first Python worker](https://workers.wasmlabs.dev/docs/languages/python)
* [Create your first Ruby worker](https://workers.wasmlabs.dev/docs/languages/ruby)
* [Create your first Go worker](https://workers.wasmlabs.dev/docs/languages/go)
* [Create your first JavaScript worker](https://workers.wasmlabs.dev/docs/languages/javascript)

## Documentation

All our documentation is available at <https://workers.wasmlabs.dev>.

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
