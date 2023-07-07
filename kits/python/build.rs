// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // TO-DO: Two approaches:
    // 1) Use python.wasm directly from WLR.
    // In the future, it must be rebuilt including new host functions required for wws (ie: send_http_request, etc.)
    const URL: &str = "https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/python%2F3.11.3%2B20230428-7d1b259/python-3.11.3.wasm";
    const FILE: &str = "python.wasm";

    let response = reqwest::blocking::get(URL).expect("FATAL! Request failed!");
    let body = response.text().expect("FATAL! Can't read request body!");
    let mut output_file = File::create(FILE).expect("FATAL! Failed to create file!");
    io::copy(&mut body.as_bytes(), &mut output_file).expect("FATAL! Failed to copy content into output file!");

    // 2) Use wrl_libpy, but it might fail depending on the underlying HW architecture (ie: arm64)
    use wlr_libpy::bld_cfg::configure_static_libs;
    configure_static_libs().unwrap().emit_link_flags();
}
