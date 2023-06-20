# JavaScript environment variables example

Run a JavaScript that returns a JSON output based on an environment variable.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/js-json"
```

## Resources

* [Environment variables](https://workers.wasmlabs.dev/docs/features/environment-variables)
* [JavaScript documentation](https://workers.wasmlabs.dev/docs/languages/javascript)
