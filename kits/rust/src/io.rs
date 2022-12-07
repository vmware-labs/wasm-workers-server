// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::Content;
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
    #[serde(default)]
    params: HashMap<String, String>,
}

impl Input {
    /// Build the object from a JSON input
    pub fn new(reader: Stdin) -> Result<Self> {
        serde_json::from_reader::<Stdin, Input>(reader).map_err(anyhow::Error::new)
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

    /// Retrieve the paramaters
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }
}

/// Represents the JSON output that the worker must return
/// back to the main project
#[derive(Serialize, Deserialize)]
pub struct Output {
    data: String,
    headers: HashMap<String, String>,
    status: u16,
    kv: HashMap<String, String>,
    base64: bool,
}

impl Output {
    /// Build the struct from Scratch
    pub fn new(
        data: &str,
        status: u16,
        headers: Option<HashMap<String, String>>,
        kv: Option<HashMap<String, String>>,
        base64: bool,
    ) -> Self {
        Self {
            data: data.to_string(),
            status,
            headers: headers.unwrap_or_default(),
            kv: kv.unwrap_or_default(),
            base64,
        }
    }

    /// Build the struct from a http::Response object
    pub fn from_response(response: Response<Content>, cache: HashMap<String, String>) -> Self {
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
        let content = response.into_body();
        let body;
        let base64;

        match content {
            Content::Base64(data) => {
                body = data;
                base64 = true;
            }
            Content::Text(data) => {
                body = data;
                base64 = false;
            }
        }

        Self::new(&body, status, Some(headers), Some(cache), base64)
    }

    /// Convert it to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).map_err(anyhow::Error::new)
    }
}
