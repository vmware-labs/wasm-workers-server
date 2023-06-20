# Ruby environment variables example

Run a Ruby worker that uses environment variables in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/ruby-envs"
```

## Resources

* [Environment variables](https://workers.wasmlabs.dev/docs/features/environment-variables)
* [Ruby documentation](https://workers.wasmlabs.dev/docs/languages/ruby)
