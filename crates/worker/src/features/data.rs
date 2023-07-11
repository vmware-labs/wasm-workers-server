// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use wws_data_kv::KVConfigData;

/// Configure a data plugin for the worker
#[derive(Deserialize, Clone, Default)]
pub struct ConfigData {
    /// Creates a Key/Value store associated to the given worker
    pub kv: Option<KVConfigData>,
}
