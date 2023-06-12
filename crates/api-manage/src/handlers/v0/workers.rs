// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::models::{Worker, WorkerConfig};
use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};
use wws_router::Routes;

/// Return the list of loaded workers.
#[utoipa::path(
    responses(
        (status = 200, description = "Returns all the workers", body = [Worker])
    )
)]
#[get("/_api/v0/workers")]
pub async fn handle_api_workers(routes: Data<Routes>) -> Result<impl Responder> {
    let workers: Vec<Worker> = routes.routes.iter().map(Worker::from).collect();

    Ok(Json(workers))
}

/// Return the details of a specific worker. It includes all the configuration details
#[utoipa::path(
    responses(
        (status = 200, description = "Return the configuration associated to the given worker", body = [WorkerConfig]),
        (status = 404, description = "The worker is not present")
    ),
    params(
        ("id" = String, Path, description = "Worker identifier"),
    )
)]
#[get("/_api/v0/workers/{id}")]
pub async fn handle_api_worker(routes: Data<Routes>, path: Path<String>) -> HttpResponse {
    let worker = routes
        .routes
        .iter()
        .find(|r| &r.worker.id == path.as_ref())
        .map(|r| &r.worker);

    if let Some(worker) = worker {
        HttpResponse::Ok().json(WorkerConfig::from(worker))
    } else {
        HttpResponse::NotFound().json("{}")
    }
}
