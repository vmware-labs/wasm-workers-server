// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct HttpRequests {
    /// List of allowed domains to perform the calls
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
    /// List of allowed HTTP methods for the worker
    #[serde(default = "default_methods")]
    pub allowed_methods: Vec<String>,
    /// Allow HTTP requests
    #[serde(default = "default_false")]
    pub allow_http: bool,
}

/// Turn the given configuration false by default
fn default_false() -> bool {
    false
}

/// It allows only basic methods by default
fn default_methods() -> Vec<String> {
    Vec::from([
        String::from("GET"),
        String::from("POST"),
        String::from("PUT"),
        String::from("PATCH"),
        String::from("DELETE"),
    ])
}
