// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod args;
mod expand;

use proc_macro::TokenStream;

// General worker entrypoint. It will bind the input
// with Request and Response objects
#[proc_macro_attribute]
pub fn worker(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand::expand_macro(attr, item)
}
