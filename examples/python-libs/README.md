# Python + libraries example

Run a Python worker that uses a Python library in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* Clone the repository:

  ```shell-session
  git clone https://github.com/vmware-labs/wasm-workers-server.git &&
    cd ./wasm-workers-server/examples/python-libs
  ```

* Install the Python libraries

  ```shell-session
  pip3 install -r requirements.txt -t ./_libs
  ```

## Run

This example runs from the previously cloned repository (See [Prerequisites](#prerequisites)). Make sure you followed all the steps and you're in the `examples/python-libs` folder:

```shell-session
wws .
```

## Resources

* [Python documentation](https://workers.wasmlabs.dev/docs/languages/python)
