// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod bindings;
mod error;

use bindings::load_bindings_into_global;
use javy::{json, Runtime};
use once_cell::sync::OnceCell;
use regex::Regex;
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
};

// Load bindings from WIT file.
wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});

/// Ready to use runtime + polyfill
static mut RUNTIME: OnceCell<Runtime> = OnceCell::new();

// JS polyfill
static POLYFILL: &str = include_str!("../shims/dist/index.js");

/// Preinitialize the module with Wizer
#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    let runtime = Runtime::default();

    // Precompile the Polyfill to bytecode
    let context = runtime.context();
    let bytecode = context.compile_global("polyfill.js", POLYFILL).unwrap();

    // Preload it
    let _ = context.eval_binary(&bytecode);

    // Store result
    unsafe { RUNTIME.set(runtime).unwrap() };
}

/// Determine the worker JS type
enum JSWorkerType {
    /// Relies on the global scope. No ECMA modules.
    Global,
    /// Exports a default function which is the one replying to the events.
    DefaultExport,
}

/// Identify the worker source code to run it properly.
fn identify_type(src: &str) -> JSWorkerType {
    // Detect default exported functions and objects
    let default_regex = Regex::new(r"(?-u)export\s+default\s+\w+;?").unwrap();
    // Detect default exported object
    let default_block_regex = Regex::new(r"export\s+default\s*\{([\s\n\r]*.*)+\};?").unwrap();
    // Detect exported functions with the "as" syntax like "export { app as default }";
    let default_as_regex =
        Regex::new(r"(?-u)export\s*\{[\s\n\r]*\w+\s+(as default){1}[\s\n\r]*\};?").unwrap();

    if default_regex.is_match(src)
        || default_block_regex.is_match(src)
        || default_as_regex.is_match(src)
    {
        JSWorkerType::DefaultExport
    } else {
        JSWorkerType::Global
    }
}

fn main() {
    let runtime = unsafe { RUNTIME.get().unwrap() };
    let context = runtime.context();

    let source = fs::read_to_string("/src/index.js");
    let mut contents = String::new();
    let mut request = String::new();

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

    // Checks if the given code uses ECMAScript modules. Currently, we don't plan to integrate
    // a full JavaScript parser for this, so we are going to rely on regexps. This implementation
    // has limitations like detecting "// export default app;" as a module. In the future,
    // we may add more complete checks.
    match identify_type(&contents) {
        JSWorkerType::DefaultExport => {
            let _ = context.eval_module("handler.mjs", &contents).unwrap();
            let _ = context
                .eval_module(
                    "runtime.mjs",
                    &format!("import {{ default as handler }} from 'handler.mjs'; addEventListener('fetch', (e) => {{ e.respondWith(handler.fetch(e.request)) }});"),
                )
                .unwrap();
        }
        _ => {
            context.eval_global("handler.js", &contents).unwrap();
        }
    }

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
