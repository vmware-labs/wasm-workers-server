// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

/// List of runtime errors
#[derive(Debug)]
pub enum RuntimeError {
    InvalidBinding { invalid_export: String },
}
