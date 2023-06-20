# Ruby Key / Value store example

Run a Ruby worker that uses a Key / Value store in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/ruby-kv"
```

## Resources

* [Key / Value store](https://workers.wasmlabs.dev/docs/features/key-value)
* [Ruby documentation](https://workers.wasmlabs.dev/docs/languages/ruby)
