// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{self, Result};

use actix_web::{
    http::{header::HeaderMap, StatusCode},
    HttpRequest,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// JSON input for wasm modules. This information is passed via STDIN / WASI
/// to the module.
#[derive(Serialize, Deserialize)]
pub struct WasmInput<'a> {
    /// Request full URL
    url: &'a str,
    /// Request method
    method: &'a str,
    /// Request headers
    headers: HashMap<String, String>,
    /// Request body
    body: &'a str,
    /// Key / Value store content if available
    kv: HashMap<String, String>,
    /// The list of parameters in the URL
    params: HashMap<String, String>,
}

impl<'a> WasmInput<'a> {
    /// Generates a new struct to pass the data to wasm module. It's based on the
    /// HttpRequest, body and the Key / Value store (if available)
    pub fn new(
        request: &'a HttpRequest,
        body: &'a str,
        kv: Option<HashMap<String, String>>,
    ) -> Self {
        let mut params = HashMap::new();

        for (k, v) in request.match_info().iter() {
            params.insert(k.to_string(), v.to_string());
        }

        let url = match request.uri().path_and_query() {
            Some(path) => path.as_str(),
            None => request.uri().path(),
        };

        Self {
            url,
            method: request.method().as_str(),
            headers: Self::build_headers_hash(request.headers()),
            body,
            kv: kv.unwrap_or_default(),
            params,
        }
    }

    /// Create HashMap from a HeadersMap
    fn build_headers_hash(headers: &HeaderMap) -> HashMap<String, String> {
        let mut parsed_headers = HashMap::new();

        for (key, value) in headers.iter() {
            parsed_headers.insert(
                String::from(key.as_str()),
                String::from(value.to_str().unwrap()),
            );
        }

        parsed_headers
    }
}

/// JSON output from a wasm module. This information is passed via STDOUT / WASI
/// from the module.
#[derive(Deserialize, Debug)]
pub struct WasmOutput {
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response HTTP status
    pub status: u16,
    /// New state of the K/V store if available
    pub kv: HashMap<String, String>,
    /// Response body data
    data: String,
    /// Internal value to indicate if the body is base64 encoded
    #[serde(default = "default_base64_encoding")]
    base64: bool,
}

fn default_base64_encoding() -> bool {
    false
}

impl WasmOutput {
    /// Initializes a new WasmOutput object
    pub fn new(
        body: &str,
        headers: HashMap<String, String>,
        status: u16,
        kv: HashMap<String, String>,
    ) -> Self {
        Self {
            data: String::from(body),
            base64: false,
            headers,
            status,
            kv,
        }
    }

    /// Build a default WasmOutput for a failed run. It will
    /// return a generic error message and the proper 503
    /// status code
    pub fn failed() -> Self {
        Self::new(
            "<p>There was an error running this function</p>",
            HashMap::from([("content-type".to_string(), "text/html".to_string())]),
            StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            HashMap::new(),
        )
    }

    /// Return the content body as bytes. It will automatically
    /// decode the data if the base64 flag is enabled.
    pub fn body(&self) -> Result<Vec<u8>> {
        if self.base64 {
            Ok(general_purpose::STANDARD
                .decode(&self.data)
                .map_err(|_| errors::WorkerError::WorkerBodyReadError)?)
        } else {
            Ok(self.data.as_bytes().into())
        }
    }
}
