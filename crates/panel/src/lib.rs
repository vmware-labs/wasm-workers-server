// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use actix_web::web::ServiceConfig;

mod handlers;

/// Add the administration panel HTTP handlers to an existing
/// Actix application.
pub fn config_panel_handlers(cfg: &mut ServiceConfig) {
    cfg.service(handlers::panel::handle_static_panel);
}
