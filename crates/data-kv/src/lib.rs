// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod store;

use serde::Deserialize;
use std::collections::HashMap;
use store::KVStore;

/// The Key/Value store configuration. This information is read from workers TOML files.
#[derive(Deserialize, Clone)]
pub struct KVConfigData {
    /// The namespace the worker will access in the global Key / Value store
    pub namespace: String,
}

/// An in-memory Key/Value store. It contains multiple namespaces which has their
/// own K/V store inside. This is used to scope the data workers can access
pub struct KV {
    /// The available K/V stores
    pub stores: HashMap<String, KVStore>,
}

impl KV {
    /// Creates a K/V store for the given namespace. If there's an existing store,
    /// this method won't apply any change.
    pub fn create_store(&mut self, namespace: &str) {
        self.stores
            .entry(namespace.to_string())
            .or_insert_with(|| KVStore::new(namespace.to_string()));
    }

    /// Replaces the content of an existing store. If the store doesn't exist,
    /// this method won't apply any change
    pub fn replace_store(&mut self, namespace: &str, state: &HashMap<String, String>) {
        if let Some(store) = self.find_mut_store(namespace) {
            store.replace(state.clone());
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

impl Default for KV {
    /// Creates a new KV instance. It initializes the K/V stores to an empty HashMap
    fn default() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }
}
