// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use utoipa::ToSchema;
use wws_router::{Route, WORKERS};

#[derive(Serialize, ToSchema)]
/// Defines a worker in a given application.
pub struct Worker {
    /// Worker identifier
    id: String,
    /// The associated name to this worker
    #[schema(example = "default")]
    name: String,
    /// API path for this specific worker.
    #[schema(example = "/api/hello")]
    path: String,
    /// Associated source code / wasm module to this worker
    #[schema(example = "/app/api/hello.js")]
    filepath: String,
}

impl From<&Route> for Worker {
    fn from(value: &Route) -> Self {
        let workers = WORKERS
            .read()
            .expect("error locking worker lock for reading");
        let name = workers
            .get(&value.worker)
            .expect("unexpected missing worker")
            .config
            .name
            .as_ref();

        Self {
            id: value.worker.clone(),
            name: name.unwrap_or(&String::from("default")).to_string(),
            path: value.path.clone(),
            filepath: value.handler.to_string_lossy().to_string(),
        }
    }
}
