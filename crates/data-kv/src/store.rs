// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// A K/V store. It's composed by a namespace and the list of Key/Values
pub struct KVStore {
    /// The namespace associated to this Key/Value store
    pub namespace: String,
    /// The list of Key - Values. In this project, both keys and values are considered
    /// strings.
    store: HashMap<String, String>,
}

impl KVStore {
    /// Creates a new K/V store for the given namespace
    pub fn new(namespace: String) -> Self {
        Self {
            namespace,
            store: HashMap::new(),
        }
    }

    /// Clone the current content of the Key/Value store
    pub fn clone(&self) -> HashMap<String, String> {
        self.store.clone()
    }

    /// Replace the content of the K/V store with a new state
    pub fn replace(&mut self, state: HashMap<String, String>) {
        self.store = state;
    }
}
