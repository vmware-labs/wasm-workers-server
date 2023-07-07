// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{error::RuntimeError};

/// Defines the different bindings required for the worker.
/// It includes utilities to log information, make HTTP requests,
/// and more in the future.
///
/// It applies them to the global context as __wws_X variables.
pub fn load_bindings_into_global() -> Result<(), RuntimeError> {
    Ok(())
}
