// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

/// List of runtime errors
pub enum RuntimeError {
    InvalidBinding { invalid_export: String },
}
