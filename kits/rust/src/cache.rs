// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

/// A cache system based on snapshots.
pub type Cache = HashMap<String, String>;
