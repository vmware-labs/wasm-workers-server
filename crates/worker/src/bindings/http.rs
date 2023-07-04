// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use reqwest::Method;
use tokio::runtime::Builder;

// Implement the HTTP bindings for the workers.
wit_bindgen_wasmtime::export!({paths: ["../../wit/core/http.wit"]});
use http::{Http, HttpError, HttpMethod, HttpRequest, HttpRequestError, HttpResponse};

pub use http::add_to_linker;

pub struct HttpBindings {}

impl HttpBindings {
    /// Map the reqwest error to a known http-error
    fn map_reqwest_err(e: &reqwest::Error) -> HttpError {
        if e.is_timeout() {
            HttpError::Timeout
        } else if e.is_redirect() {
            HttpError::RedirectLoop
        } else if e.is_request() {
            HttpError::InvalidRequest
        } else if e.is_body() {
            HttpError::InvalidRequestBody
        } else if e.is_decode() {
            HttpError::InvalidResponseBody
        } else {
            HttpError::InternalError
        }
    }
}

impl Http for HttpBindings {
    fn send_http_request(
        &mut self,
        req: HttpRequest<'_>,
    ) -> Result<HttpResponse, HttpRequestError> {
        // Create local variables from the request
        let mut headers = Vec::new();
        let uri = req.uri.to_string();
        let body = req.body.unwrap_or(&[]).to_vec();

        for (key, value) in req.headers {
            headers.push((key.to_string(), value.to_string()));
        }

        // Run the request in an async thread
        let thread_result = std::thread::spawn(move || {
            Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let client = reqwest::Client::new();

                    let method = match req.method {
                        HttpMethod::Get => Method::GET,
                        HttpMethod::Post => Method::POST,
                        HttpMethod::Put => Method::PUT,
                        HttpMethod::Patch => Method::PATCH,
                        HttpMethod::Delete => Method::DELETE,
                        HttpMethod::Options => Method::OPTIONS,
                        HttpMethod::Head => Method::HEAD,
                    };

                    let mut builder = client.request(method, uri);

                    for (key, value) in headers {
                        builder = builder.header(key, value);
                    }

                    builder = builder.body(body);

                    match builder.send().await {
                        Ok(res) => {
                            let mut headers = Vec::new();
                            let status = res.status().as_u16();

                            for (name, value) in res.headers().iter() {
                                headers
                                    .push((name.to_string(), value.to_str().unwrap().to_string()));
                            }

                            let body = res.bytes().await;

                            Ok(HttpResponse {
                                headers,
                                status,
                                body: Some(body.unwrap().to_vec()),
                            })
                        }
                        Err(e) => {
                            // Manage the different possible errors from Reqwest
                            Err(HttpRequestError {
                                error: Self::map_reqwest_err(&e),
                                message: e.to_string(),
                            })
                        }
                    }
                })
        })
        .join();

        match thread_result {
            Ok(res) => match res {
                Ok(res) => Ok(res),
                Err(err) => Err(err),
            },
            Err(_) => Err(HttpRequestError {
                error: HttpError::InternalError,
                message: "There was an error processing the request on the host side.".to_string(),
            }),
        }
    }
}
