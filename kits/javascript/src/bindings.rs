// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::{error::RuntimeError, http::HttpError};
use javy::quickjs::{JSContextRef, JSValue, JSValueRef};

// Bindings from WIT
use crate::http;

/// Defines the different bindings required for the worker.
/// It includes utilities to log information, make HTTP requests,
/// and more in the future.
///
/// It applies them to the global context as __wws_X variables.
pub fn load_bindings_into_global(
    context: &JSContextRef,
    global: JSValueRef,
) -> Result<(), RuntimeError> {
    global
        .set_property(
            "__wws_send_http_request",
            context
                .wrap_callback(|_ctx, _this_arg, args| {
                    let uri = args[0].to_string();
                    // Options
                    let opts: HashMap<String, JSValue> = args[1].try_into()?;
                    let method = opts.get("method").unwrap().to_string();
                    let headers = opts.get("headers").unwrap();
                    let body = opts.get("body").unwrap();

                    let method = match method.as_str() {
                        "GET" => http::HttpMethod::Get,
                        "POST" => http::HttpMethod::Post,
                        // Default to GET
                        _ => http::HttpMethod::Get,
                    };

                    let mut parsed_headers: Vec<(String, String)> = Vec::new();

                    if let JSValue::Object(headers) = headers {
                        for (key, val) in headers.iter() {
                            parsed_headers.push((key.to_string(), val.to_string()));
                        }
                    }

                    let headers_slice: &[(&str, &str)] = &parsed_headers
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect::<Vec<(&str, &str)>>();

                    let parsed_body: Option<&[u8]>;

                    // The shim always return an array buffer.
                    if let JSValue::ArrayBuffer(buf) = body {
                        parsed_body = Some(buf.as_ref());
                    } else if let JSValue::String(data) = body {
                        parsed_body = Some(data.as_bytes());
                    } else {
                        parsed_body = None;
                    }

                    let req = http::HttpRequest {
                        uri: uri.as_str(),
                        body: parsed_body,
                        headers: headers_slice,
                        method,
                        params: &[],
                    };

                    match http::send_http_request(req) {
                        Ok(result) => {
                            let body = result.body.unwrap_or(Vec::new());
                            let mut headers = HashMap::new();

                            for (key, val) in result.headers.iter() {
                                headers.insert(key.as_str(), JSValue::String(val.to_string()));
                            }

                            let parsed_result = HashMap::from([
                                ("status", JSValue::Int(result.status as i32)),
                                ("body", JSValue::ArrayBuffer(body)),
                                ("headers", JSValue::from_hashmap(headers)),
                            ]);

                            Ok(JSValue::from_hashmap(parsed_result))
                        }
                        Err(err) => {
                            let kind = match err.error {
                                HttpError::InvalidRequest => "Invalid Request".to_string(),
                                HttpError::InvalidRequestBody => "Invalid Request Body".to_string(),
                                HttpError::InvalidResponseBody => {
                                    "Invalid Response Body".to_string()
                                }
                                HttpError::NotAllowed => "Not allowed".to_string(),
                                HttpError::InternalError => "Internal Error".to_string(),
                                HttpError::Timeout => "Request Timeout".to_string(),
                                HttpError::RedirectLoop => "Redirect Loop".to_string(),
                            };

                            Ok(JSValue::from_hashmap(HashMap::from([
                                ("error", JSValue::Bool(true)),
                                ("type", JSValue::String(kind)),
                                ("message", JSValue::String(err.message)),
                            ])))
                        }
                    }
                })
                .map_err(|_| RuntimeError::InvalidBinding {
                    invalid_export: "send_http_request".to_string(),
                })?,
        )
        .map_err(|_| RuntimeError::InvalidBinding {
            invalid_export: "send_http_request".to_string(),
        })?;

    global
        .set_property(
            "__wws_console_log",
            context
                .wrap_callback(|_ctx, _this_arg, args| {
                    let msg = args[0].to_string();
                    // For now, just print it in STDERR
                    eprintln!("{msg}");

                    Ok(JSValue::Null)
                })
                .map_err(|_| RuntimeError::InvalidBinding {
                    invalid_export: "console_log".to_string(),
                })?,
        )
        .map_err(|_| RuntimeError::InvalidBinding {
            invalid_export: "console_log".to_string(),
        })?;

    Ok(())
}
