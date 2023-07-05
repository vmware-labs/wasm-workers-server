// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod bindings;
mod error;

use bindings::load_bindings_into_global;
use javy::{json, Runtime};
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
};

// Load bindings from WIT file.
wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});

// JS polyfill
static POLYFILL: &str = include_str!("../shims/dist/index.js");

fn main() {
    let runtime = Runtime::default();
    let context = runtime.context();

    let source = fs::read_to_string("/src/index.js");
    let mut contents = String::new();
    let mut request = String::new();
    contents.push_str(POLYFILL);

    stdin().read_to_string(&mut request).unwrap();

    // Inject global variables
    for (key, val) in env::vars() {
        let escaped_val = val.replace('"', "\\\"");
        contents.push_str(&format!("const {} = \"{}\";", key, escaped_val));
    }

    // Add the source code
    contents.push_str(&source.unwrap());

    // Add custom bindings
    let global = context.global_object().unwrap();
    match load_bindings_into_global(context, global) {
        Ok(_) => {}
        Err(e) => match e {
            // In the future we may have more errors.
            error::RuntimeError::InvalidBinding { invalid_export } => {
                eprintln!("There was an error adding the '{invalid_export}' binding");
            }
        },
    }

    context.eval_global("handler.js", &contents).unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("entrypoint").unwrap();

    let input_bytes = request.as_bytes();
    let input_value = json::transcode_input(context, input_bytes).unwrap();

    // Run the handler to get the output
    match entrypoint.call(&global, &[input_value]) {
        Ok(_) => {}
        Err(err) => eprintln!("Error calling the main entrypoint: {err}"),
    };

    if context.is_pending() {
        if let Err(err) = context.execute_pending() {
            eprintln!("Error running async methods: {err}");
        }
    }

    let global = context.global_object().unwrap();
    let error_value = global.get_property("error").unwrap();
    let output_value = global.get_property("result").unwrap();

    if !error_value.is_null_or_undefined() {
        eprintln!("{}", error_value.as_str_lossy());
    }

    let output = json::transcode_output(output_value).unwrap();

    stdout()
        .write_all(&output)
        .expect("Error when returning the response");
}
