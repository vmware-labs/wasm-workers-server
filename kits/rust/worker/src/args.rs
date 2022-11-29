// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use syn::parse::{Parse, ParseStream, Result};
use syn::{punctuated::Punctuated, Ident, Token};

/// Parse valid arguments for the worker
pub struct Args {
    idents: HashSet<Ident>,
}

impl Args {
    pub fn has_cache(&self) -> bool {
        self.idents
            .iter()
            .any(|i| i.to_string() == String::from("cache"))
    }
}

impl Parse for Args {
    fn parse(args: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(args)?;
        Ok(Args {
            idents: vars.into_iter().collect(),
        })
    }
}
