// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod bindings;
mod error;

use bindings::load_bindings_into_global;

use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
};

// Load bindings from WIT file.
wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});

fn main() {
    // TO-DO: Initialize Python runtime
    
    // load source code
    let source = fs::read_to_string("/src/index.py");
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
    match load_bindings_into_global() {
        Ok(_) => {}
        Err(e) => match e {
            // In the future we may have more errors.
            error::RuntimeError::InvalidBinding { invalid_export } => {
                eprintln!("There was an error adding the '{invalid_export}' binding");
            }
        },
    }

    // let input_bytes = request.as_bytes();
    // let input_value = json::transcode_input(context, input_bytes).unwrap();

    // // Run the handler to get the output
    // match entrypoint.call(&global, &[input_value]) {
    //     Ok(_) => {}
    //     Err(err) => eprintln!("Error calling the main entrypoint: {err}"),
    // };

    // if context.is_pending() {
    //     if let Err(err) = context.execute_pending() {
    //         eprintln!("Error running async methods: {err}");
    //     }
    // }

    let output = "{module output}";

    stdout()
        .write_all(output.as_bytes())
        .expect("Error when returning the response");
}
