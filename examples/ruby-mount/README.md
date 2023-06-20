# Ruby mount folders example

Run a Ruby worker that uses data from a mounted folder in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/ruby-mount"
```

## Resources

* [Mount folders](https://workers.wasmlabs.dev/docs/features/mount-folders)
* [Ruby documentation](https://workers.wasmlabs.dev/docs/languages/ruby)
