// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::args::Args;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Expand the given input after processing by the macro
pub fn expand_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let handler_fn = parse_macro_input!(item as syn::ItemFn);
    let handler_fn_name = &handler_fn.sig.ident;
    let args = parse_macro_input!(attr as Args);

    let func_call = if args.has_cache() {
        quote! {
            #handler_fn_name(input.to_http_request(), &mut cache)
        }
    } else {
        quote! {
            #handler_fn_name(input.to_http_request())
        }
    };

    let main_fn = quote! {
        use wasm_workers_rs::io::{Input, Output};
        use std::io::stdin;

        fn main() {
            let input = Input::new(stdin());
            let error = Output::new(
                "There was an error running the handler",
                500,
                None,
                None,
                false
            ).to_json().unwrap();

            if let Ok(input) = input {
                let mut cache = input.cache_data();

                if let Ok(response) = #func_call {
                    match Output::from_response(response, cache).to_json() {
                        Ok(res) => println!("{}", res),
                        Err(_) => println!("{}", error)
                    }
                } else {
                    println!("{}", error)
                }
            } else {
                println!("{}", error)
            }
        }

        #handler_fn
    };

    TokenStream::from(main_fn)
}
