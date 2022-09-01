// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use http::Response;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::Stdin;

/// Represents the JSON data that will be injected by the
/// main project.
#[derive(Serialize, Deserialize)]
pub struct Input {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: String,
    kv: HashMap<String, String>,
}

impl Input {
    /// Build the object from a JSON input
    pub fn new(reader: Stdin) -> Result<Self> {
        serde_json::from_reader::<Stdin, Input>(reader).map_err(|e| anyhow::Error::new(e))
    }

    /// Convers the current object to a valid http::Request
    /// object
    pub fn to_http_request(&self) -> http::Request<String> {
        let mut request = http::request::Builder::new()
            .uri(&self.url)
            .method(self.method.as_str());

        for (key, value) in self.headers.iter() {
            request = request.header(key, value);
        }

        request.body(self.body.to_string()).unwrap()
    }

    /// Retrieve the Key/Value data
    pub fn cache_data(&self) -> HashMap<String, String> {
        self.kv.clone()
    }
}

/// Represents the JSON output that the handler must return
/// back to the main project
#[derive(Serialize, Deserialize)]
pub struct Output {
    body: String,
    headers: HashMap<String, String>,
    status: u16,
    kv: HashMap<String, String>,
}

impl Output {
    /// Build the struct from Scratch
    pub fn new(
        body: &str,
        status: u16,
        headers: Option<HashMap<String, String>>,
        kv: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            body: body.to_string(),
            status: status,
            headers: headers.unwrap_or_else(|| HashMap::new()),
            kv: kv.unwrap_or_else(|| HashMap::new()),
        }
    }

    /// Build the struct from a http::Response object
    pub fn from_response(response: Response<String>, cache: HashMap<String, String>) -> Self {
        let mut headers = HashMap::new();

        for (key, value) in response.headers().iter() {
            headers.insert(
                String::from(key.as_str()),
                String::from(value.to_str().unwrap()),
            );
        }

        // Note: added status here because `into_body` takes ownership of the
        // response
        let status = response.status().as_u16();

        Self::new(
            response.into_body().as_str(),
            status,
            Some(headers),
            Some(cache.clone()),
        )
    }

    /// Convert it to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(|e| anyhow::Error::new(e))
    }
}
