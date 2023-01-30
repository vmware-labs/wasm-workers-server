// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::runtimes::Runtimes;
use clap::Subcommand;

/// Available subcommands in the CLI
#[derive(Subcommand, Debug)]
pub enum Main {
    #[clap(name = "runtimes")]
    Runtimes(Runtimes),
}
