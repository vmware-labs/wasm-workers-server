// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use std::collections::HashMap;

/// The Key/Value store configuration. This information is read from handlers TOML files.
#[derive(Deserialize, Clone)]
pub struct KVConfigData {
    /// The namespace the handler will access in the global Key / Value store
    pub namespace: String,
}

/// An in-memory Key/Value store. It contains multiple namespaces which has their
/// own K/V store inside. This is used to scope the data handlers can access
pub struct KV {
    /// The available K/V stores
    pub stores: HashMap<String, KVStore>,
}

impl KV {
    /// Creates a new KV instance. It initializes the K/V stores to an empty HashMap
    pub fn new() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }

    /// Creates a K/V store for the given namespace. If there's an existing store,
    /// this method won't apply any change.
    pub fn create_store(&mut self, namespace: &str) {
        self.stores
            .entry(namespace.to_string())
            .or_insert_with(|| KVStore::new(namespace.to_string()));
    }

    /// Replaces the content of an existing store. If the store doesn't exist,
    /// this method won't apply any change
    pub fn replace_store(&mut self, namespace: &str, state: HashMap<String, String>) {
        if let Some(store) = self.find_mut_store(namespace) {
            store.replace(state);
        }
    }

    /// Look for the store related to the given namespace. This will return a reference
    /// to the desired store if available
    pub fn find_store(&self, namespace: &str) -> Option<&KVStore> {
        self.stores.get(namespace)
    }

    /// Similar to `find_store`, but it returns a mutable reference. It's useful
    /// to write information to the given store
    pub fn find_mut_store(&mut self, namespace: &str) -> Option<&mut KVStore> {
        self.stores.get_mut(namespace)
    }
}

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
