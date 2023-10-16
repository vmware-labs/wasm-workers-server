# JavaScript Hono example

Run a JavaScript application that uses the [HonoJS framework](https://hono.dev/) in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/js-hono/dist"
```

## Resources

* [JavaScript documentation](https://workers.wasmlabs.dev/docs/languages/javascript)
