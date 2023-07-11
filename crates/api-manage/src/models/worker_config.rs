// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use serde::Serialize;
use utoipa::ToSchema;
use wws_worker::{
    features::{data::ConfigData, folders::Folder},
    Worker,
};

#[derive(Serialize, ToSchema)]
/// Defines a worker in a given application.
pub struct WorkerConfig {
    /// The worker identifier
    #[schema(example = "default")]
    id: String,
    /// The associated name to this worker
    #[schema(example = "default")]
    name: String,
    /// Version of the configuration file
    #[schema(example = "/api/hello")]
    version: String,
    /// Associated data configuration
    pub data: WorkerConfigData,
    /// Mounted folders
    pub folders: Vec<WorkerFolder>,
    /// Environment variables
    pub vars: HashMap<String, String>,
}

impl From<&Worker> for WorkerConfig {
    fn from(value: &Worker) -> Self {
        let config = &value.config;

        let folders = config
            .folders
            .as_ref()
            .map(|f| {
                f.iter()
                    .map(WorkerFolder::from)
                    .collect::<Vec<WorkerFolder>>()
            })
            .unwrap_or(Vec::new());

        Self {
            id: value.id.clone(),
            name: config
                .name
                .as_ref()
                .unwrap_or(&String::from("default"))
                .to_string(),
            version: config.version.clone(),
            data: WorkerConfigData::from(config.data.as_ref()),
            folders,
            vars: config.vars.clone(),
        }
    }
}

#[derive(Serialize, ToSchema)]
/// Data configuration for this specific worker
pub struct WorkerConfigData {
    /// Key/Value namespace this worker can read/write
    kv: Option<String>,
}

impl From<Option<&ConfigData>> for WorkerConfigData {
    fn from(value: Option<&ConfigData>) -> Self {
        Self {
            kv: value
                .map(|data| data.kv.as_ref().map(|kv| kv.namespace.clone()))
                .unwrap_or(None),
        }
    }
}

#[derive(Serialize, ToSchema)]
/// Data configuration for this specific worker
pub struct WorkerFolder {
    /// Filesystem path to mount in the worker
    #[schema(example = "/tmp/worker-dir")]
    from: String,
    /// Worker internal location for this specific folder
    #[schema(example = "/tmp")]
    to: String,
}

impl From<&Folder> for WorkerFolder {
    fn from(value: &Folder) -> Self {
        Self {
            from: value.from.to_string_lossy().to_string(),
            to: value.to.clone(),
        }
    }
}
