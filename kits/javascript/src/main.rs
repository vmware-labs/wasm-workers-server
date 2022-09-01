// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use quickjs_wasm_rs::{json, Context};
use std::io::{self, stdin, stdout, Read, Write};

// JS polyfill
static POLYFILL: &str = include_str!("./glue.js");

// Separator between source code and request data
static SEPARATOR: &str = "[[[input]]]";

fn main() {
    let mut context = Context::default();
    context
        .register_globals(io::stderr(), io::stderr())
        .unwrap();

    let mut contents = String::new();
    let mut source = String::new();
    contents.push_str(&POLYFILL);

    stdin().read_to_string(&mut source).unwrap();
    let chunks: Vec<&str> = source.split(SEPARATOR).collect();

    contents.push_str(&chunks.first().unwrap());

    let _ = context.eval_global("handler.js", &contents).unwrap();
    let global = context.global_object().unwrap();
    let entrypoint = global.get_property("entrypoint").unwrap();

    let input_bytes = &chunks.last().unwrap().as_bytes();
    let input_value = json::transcode_input(&context, &input_bytes).unwrap();

    // Run the handler to get the output
    let output_value = match entrypoint.call(&global, &[input_value]) {
        Ok(result) => result,
        Err(err) => panic!("{}", err.to_string()),
    };

    let output = json::transcode_output(output_value).unwrap();

    stdout()
        .write(&output)
        .expect("Error when returning the response");
    stdout().flush().expect("Error when returning the response");
}
