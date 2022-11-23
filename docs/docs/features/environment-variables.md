---
sidebar_position: 3
---

# Environment Variables

Wasm Workers Server allows you to configure environment variables so a worker can read them. This configuration is done through the `TOML` file associated to a worker (a `TOML` file with the same filename as the worker). **This means every worker has its own set of environment variables**.

The following `TOML` config file adds a new `JSON_MESSAGE` environment variable:

```toml
name = "json"
version = "1"

[vars]
JSON_MESSAGE = "Hello 👋! This message comes from an environment variable"
```

Then, you can read them in your worker:

* [Read environment variables in a JavaScript worker](../tutorials/javascript-workers.md#read-environment-variables)
* [Read environment variables in a Rust worker](../tutorials/rust-workers.md#read-environment-variables)

## Inject existing environment variables

You can inject existing environment variables from your current context. In this case, the value of the configured variable will be the name of the existing one with the `$` prefix.

For example, let's configure the `TOKEN` variable for the previous worker. This variable will get its value from the `TOKEN` environment variable in the current context:

```toml
name = "json"
version = "1"

[vars]
JSON_MESSAGE = "Hello 👋! This message comes from an environment variable"
TOKEN = "$TOKEN"
```

This feature allows you to configure environment variables dynamically.