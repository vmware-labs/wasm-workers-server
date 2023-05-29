// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use javy::{json, Runtime};
use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
};

// JS polyfill
static POLYFILL: &str = include_str!("./glue.js");

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

    contents.push_str(&source.unwrap());

    let _ = context.eval_global("handler.js", &contents).unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("entrypoint").unwrap();

    let input_bytes = request.as_bytes();
    let input_value = json::transcode_input(context, input_bytes).unwrap();

    // Run the handler to get the output
    match entrypoint.call(&global, &[input_value]) {
        Ok(_) => {}
        Err(err) => panic!("{}", err.to_string()),
    };

    if context.is_pending() {
        if let Err(err) = context.execute_pending() {
            panic!("{}", err.to_string());
        }
    }

    let global = context.global_object().unwrap();
    let output_value = global.get_property("result").unwrap();

    let output = json::transcode_output(output_value).unwrap();

    stdout()
        .write_all(&output)
        .expect("Error when returning the response");
}
