---
title: Machine Learning inference
---

:::caution

This is a feature preview. It will be available in v1.5.0

:::

Artificial Intelligence (AI) and Machine Learning (ML) are hot topics in the community. This feature enables you to expand the capabilities of your workers by running ML models in your models. For example, you can develop an application that uses image classification or text-to-speech.

To provide this feature, Wasm Workers Server relies on the [WASI-NN proposal](https://github.com/WebAssembly/wasi-nn). This proposal defines a set of APIs to send and retrieve data, and run the ML inference at the host side. The main benefits of this approach are to reuse the existing ML ecosystem (like Tensorflow and OpenVINO) and use hardware acceleration when it's available (GPUs, TPUs, etc.).

## Available backends

A backend or ML engine is an application that parses the ML model, loads the inputs, runs them and returns the output. There are multiple backends like PyTorch, [Tensorflow](https://www.tensorflow.org/) (and [Lite version](https://www.tensorflow.org/lite)), [ONNX](https://onnxruntime.ai/) and [OpenVINO™](https://docs.openvino.ai/).

Currently, Wasm Workers Server only supports [OpenVINO™](https://docs.openvino.ai/) as ML inference engine or backend. The community is actively working on adding support for more backends, so you may expect new backends in the future.

## Prerequisites

### Install OpenVINO

Install the [OpenVINO™ Runtime (2023.0.1)](https://docs.openvino.ai/2023.0):

  * [Windows](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_windows.html)
  * [Linux](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_linux.html)
  * [MacOS](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_macos.html)

Configure the OpenVINO™ environment:

  * [Windows](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_windows.html#step-2-configure-the-environment)
  * [Linux](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_linux.html#step-2-configure-the-environment)
  * [MacOS](https://docs.openvino.ai/2023.0/openvino_docs_install_guides_installing_openvino_from_archive_macos.html#step-2-configure-the-environment)

## Run ML inference in a worker

By default, workers cannot access the WASI-NN bindings. You need to configure it using the worker configuration file. For that, create a TOML file with the same name as the worker (like `index.wasm` and `index.toml`), and configure the WASI-NN feature:

```toml
name = "wasi-nn"
version = "1"

[features]
[features.wasi_nn]
allowed_backends = ["openvino"]

[[folders]]
from = "./_models"
to = "/tmp/model"
```

In this specific configuration, we assume you are mounting a `_models` folder that contains your ML models. You need to adapt it to your specific case.

### Example

You can find a [full working example in the project repository](https://github.com/vmware-labs/wasm-workers-server/tree/main/examples/rust-wasi-nn). In this example, you have a worker that returns a website to upload an image. When you upload it, a second worker retrieves the image and runs a [MobileNet](https://arxiv.org/abs/1704.04861) ML model to classify the content of the image.

We recommend to check this example to get started with ML and Wasm Workers Server.

<img alt="The sample application showing an image with a dog. The model predicts the image contains a 'Labrador retriever' with high confidence" src="/img/docs/features/wasi-nn.webp" style={{ width: "60%", margin: "2rem auto", display: "block" }}/>

## Language compatibility

| Language | Machine learning inference |
| --- | --- |
| JavaScript | ❌ |
| Rust | ✅ |
| Go | ❌ |
| Ruby | ❌ |
| Python | ❌ |
