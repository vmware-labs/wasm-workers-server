---
sidebar_position: 1
---

# Introduction

## What's Wasm Workers Server?

Wasm Workers Server is a framework that allows you to develop and run serverless applications using a lightweight construct called "workers", explained later in the document. The server itself is implemented as a self-contained binary that routes HTTP requests to a WebAssembly runtime that hosts the workers. It looks for `.wasm` and other compatible modules (like JavaScript files) in the given folder and associate HTTP endpoints based on their path:

```bash
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

You don't need to configure anything by default. Just drop your workers in a folder and run the project to get an HTTP server and start serving requests 🚀.

```bash
$ curl http://localhost:8080/api/hello

Hello Wasm!
```

That's all! Now it's your turn [to download and start using Wasm Workers Server](./quickstart.md).

## What's a worker?

Worker has many definitions in the software ecosystem. In the context of the Web Platform, **a worker is a resource that listens to events and replies to them**. In our context, **a worker is a script or function that receives an HTTP request and returns an HTTP response**.

Applications can be developed by combining multiple workers. Every worker listens to certain events and provides responses to them. Splitting large applications into smaller pieces has several benefits:

* **Easier to develop**: workers are meant to be small and focused.
* **Easier to test**: every worker can be tested separately and the surface to cover with testing is way smaller.
* **Easier to deploy**: new platforms are focusing on workers and deploying existing applications is just a single command.

This concept may sound familiar to you. [Serverless computing](https://en.wikipedia.org/wiki/Serverless_computing) is a popular model to build web applications. Services like [AWS Lambda](https://aws.amazon.com/lambda/) and [Google Cloud Functions](https://cloud.google.com/functions) implement this model.

Workers is an implementation of serverless. Many of the existing serverless platforms run functions in a centralized infrastructure where related services are close to them. With the workers model, platforms like [Cloudflare Workers®](https://workers.cloudflare.com/), [Deno Deploy](https://deno.com/deploy), [Vercel](https://vercel.com/), [Fermyon](https://www.fermyon.com/), [Suborbital](https://suborbital.dev/) and [wasmCloud](https://wasmcloud.dev) deploy your workers and actors close to your users. For this task, they created a new set of "edge runtimes" that allows them to quickly distribute your workers around the globe.

## Why Wasm Workers Server?

Wasm Workers Server is a lightweight implementation of a Worker platform that aims for compatibility. You can use it to develop applications locally quickly or to host your applications on servers that you control entirely. It is extremely easy to get started: just download a binary and start writing your workers. You can create them based on different languages and run them securely thanks to [WebAssembly](https://webassembly.org/).

It aims for compatibility and follows an ongoing specification that different companies are working under the name of [WinterCG](https://wintercg.org/faq). This working group aims to create a common API for using Web Platform APIs like workers outside of the browser.

Many of the platforms mentioned earlier follow a similar approach, so any code you write for Wasm Workers Server can be moved to those platforms easily (or the other way around!). Remember that our focus with wws is simplicity and compatibility. Since this is a growing ecosystem, we want you to start quickly and move wherever you need.