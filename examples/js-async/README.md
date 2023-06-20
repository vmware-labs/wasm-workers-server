# JavaScript async worker example

Run a JavaScript that uses an async function in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/js-async"
```

## Resources

* [JavaScript documentation](https://workers.wasmlabs.dev/docs/languages/javascript)
