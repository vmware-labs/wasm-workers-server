# Rust WASI-NN example

Compile a Rust that performs Machine Learning (ML) inference with [WASI-NN](https://github.com/WebAssembly/wasi-nn) to WebAssembly and run it in Wasm Workers Server.

## Prerequisites

* Wasm Workers Server (wws):

  ```shell-session
  curl -fsSL https://workers.wasmlabs.dev/install | bash
  ```

* [Install Rust with rustup](https://www.rust-lang.org/tools/install)
* Install the `wasm32-wasi` target:

    ```shell-session
    rustup target add wasm32-wasi
    ```

* Install the [OpenVINO™ Runtime (2023.0.1)](https://docs.openvino.ai/2023.0)
  * [Windows](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_windows.html)
  * [Linux](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_linux.html)
  * [MacOS](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_macos.html)
* Configure the OpenVINO™ environment:
  * [Windows](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_windows.html#step-2-configure-the-environment)
  * [Linux](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_linux.html#step-2-configure-the-environment)
  * [MacOS](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_macos.html#step-2-configure-the-environment)
* Run the `prepare.sh` script to download the ML model:

    ```shell-session
    $ ./prepare.sh

    Downloading the model from https://github.com/intel/openvino-rs/tree/main/crates/openvino/tests/fixtures/mobilenet
    ....
    Finished!
    ```

## Build

```shell-session
cargo build --target wasm32-wasi --release && \
	cp target/wasm32-wasi/release/rust-wasi-nn.wasm ./inference.wasm
```

## Run

```shell-session
wws .
```

## Resources

* [Rust documentation](https://workers.wasmlabs.dev/docs/languages/rust)
