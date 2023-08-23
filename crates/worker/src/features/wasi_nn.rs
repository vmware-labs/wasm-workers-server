// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

pub const WASI_NN_BACKEND_OPENVINO: &str = "openvino";

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct WasiNnConfig {
    /// List of Machine Learning backends. For now, only "openvino" option is supported
    pub allowed_backends: Vec<String>,
}
