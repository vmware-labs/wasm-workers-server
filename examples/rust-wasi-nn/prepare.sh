#!/usr/bin/env bash

MODEL_GITHUB=https://github.com/intel/openvino-rs/tree/main/crates/openvino/tests/fixtures/mobilenet
MODEL=https://github.com/intel/openvino-rs/raw/main/crates/openvino/tests/fixtures/mobilenet

echo "Downloading the model from ${MODEL_GITHUB}"

wget --no-clobber $MODEL/mobilenet.bin --output-document=_models/model.bin
wget --no-clobber $MODEL/mobilenet.xml --output-document=_models/model.xml

echo "Finished!"
