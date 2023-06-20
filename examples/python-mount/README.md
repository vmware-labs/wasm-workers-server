# Python mount folders example

Run a Python worker that uses data from a mounted folder in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

## Run

```shell-session
wws https://github.com/vmware-labs/wasm-workers-server.git -i --git-folder "examples/python-mount"
```

## Resources

* [Mount folders](https://workers.wasmlabs.dev/docs/features/mount-folders)
* [Python documentation](https://workers.wasmlabs.dev/docs/languages/python)
