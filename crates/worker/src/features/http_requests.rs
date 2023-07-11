// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct HttpRequestsConfig {
    /// List of allowed domains to perform the calls
    pub allowed_hosts: Vec<String>,
    /// List of allowed HTTP methods for the worker
    pub allowed_methods: Vec<String>,
    /// Allow HTTP requests
    pub allow_http: bool,
}

impl Default for HttpRequestsConfig {
    fn default() -> Self {
        Self {
            allowed_hosts: Vec::default(),
            allowed_methods: Vec::from([
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("PATCH"),
                String::from("DELETE"),
            ]),
            allow_http: false,
        }
    }
}
