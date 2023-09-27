// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use wasmtime_wasi_nn::backend::openvino::OpenvinoBackend;
use wasmtime_wasi_nn::Backend;

/// Available Machine Learning backends
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum WasiNnBackend {
    /// None
    #[default]
    None,
    /// OpenVINO backend
    Openvino,
}

impl WasiNnBackend {
    /// Convert the given enum variant into a WASI-NN backend.
    pub fn to_backend(&self) -> Option<Backend> {
        match self {
            Self::None => None,
            Self::Openvino => Some(Backend::from(OpenvinoBackend::default())),
        }
    }
}

impl Display for WasiNnBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Openvino => write!(f, "openvino"),
        }
    }
}

/// Available providers to load Wasi NN models.
#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum WasiNnModelProvider {
    /// Load it from the local filesystem
    #[default]
    Local,
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct WasiNnModel {
    /// The provider to retrieve the given model.
    provider: WasiNnModelProvider,
    /// Backend to run this specific model
    backend: WasiNnBackend,
    /// For a local provider, it will retrieve the data from a local path.
    dir: PathBuf,
}

impl WasiNnModel {
    /// Provide the graph configuration from the current model. Depending on the
    /// provider, it may need to perform other tasks before running it.
    pub fn build_graph_data(&self, worker_path: &Path) -> (String, String) {
        match self.provider {
            WasiNnModelProvider::Local => {
                let data = if self.dir.is_relative() {
                    worker_path.parent().map(|parent| {
                        (
                            self.backend.clone().to_string(),
                            parent.join(&self.dir).to_string_lossy().to_string(),
                        )
                    })
                } else {
                    None
                };

                data.unwrap_or_else(|| {
                    // Absolute path or best effort if it cannot retrieve the parent path
                    (
                        self.backend.clone().to_string(),
                        self.dir.to_string_lossy().to_string(),
                    )
                })
            }
        }
    }
}

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct WasiNnConfig {
    /// List of Machine Learning backends. For now, only "openvino" option is supported
    pub allowed_backends: Vec<WasiNnBackend>,
    /// List of preloaded models. It allows you to get the models from different strategies.
    pub preload_models: Vec<WasiNnModel>,
}
