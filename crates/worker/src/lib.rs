// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

pub mod config;
pub mod io;

use actix_web::HttpRequest;
use anyhow::{anyhow, Result};
use config::Config;
use io::{WasmInput, WasmOutput};
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{Dir, WasiCtxBuilder};
use wws_config::Config as ProjectConfig;
use wws_runtimes::{init_runtime, Runtime};

/// A worker contains the engine and the associated runtime.
/// This struct will process requests by preparing the environment
/// with the runtime and running it in Wasmtime
pub struct Worker {
    /// Wasmtime engine to run this worker
    engine: Engine,
    /// Wasm Module
    module: Module,
    /// Worker runtime
    runtime: Box<dyn Runtime + Sync + Send>,
    /// Current config
    pub config: Config,
    /// The worker filepath
    path: PathBuf,
}

impl Worker {
    /// Creates a new Worker
    pub fn new(project_root: &Path, path: &Path, project_config: &ProjectConfig) -> Result<Self> {
        // Load configuration
        let mut config_path = path.to_path_buf();
        config_path.set_extension("toml");
        let mut config = Config::default();

        if fs::metadata(&config_path).is_ok() {
            if let Ok(c) = Config::try_from_file(config_path) {
                config = c;
            } else {
                println!("Error loading the config!");
            }
        }

        let engine = Engine::default();
        let runtime = init_runtime(project_root, path, project_config)?;
        let bytes = runtime.module_bytes()?;
        let module = Module::from_binary(&engine, &bytes)?;

        // Prepare the environment if required
        runtime.prepare()?;

        Ok(Self {
            engine,
            module,
            runtime,
            config,
            path: path.to_path_buf(),
        })
    }

    pub fn run(
        &self,
        request: &HttpRequest,
        body: &str,
        kv: Option<HashMap<String, String>>,
        vars: &HashMap<String, String>,
    ) -> Result<WasmOutput> {
        let input = serde_json::to_string(&WasmInput::new(request, body, kv)).unwrap();

        // Prepare STDIO
        let stdout = WritePipe::new_in_memory();
        let stderr = WritePipe::new_in_memory();
        let stdin = ReadPipe::from(input);

        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        // I have to use `String` as it's required by WasiCtxBuilder
        let tuple_vars: Vec<(String, String)> =
            vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        // Create the initial WASI context
        let mut wasi_builder = WasiCtxBuilder::new()
            .stdin(Box::new(stdin))
            .stdout(Box::new(stdout.clone()))
            .stderr(Box::new(stderr.clone()))
            .envs(&tuple_vars)?;

        // Mount folders from the configuration
        if let Some(folders) = self.config.folders.as_ref() {
            for folder in folders {
                if let Some(base) = &self.path.parent() {
                    let source = fs::File::open(base.join(&folder.from))?;
                    wasi_builder =
                        wasi_builder.preopened_dir(Dir::from_std_file(source), &folder.to)?;
                } else {
                    // TODO: Revisit error management on #73
                    return Err(anyhow!("The worker couldn't be initialized"));
                }
            }
        }

        // Pass to the runtime to add any WASI specific requirement
        wasi_builder = self.runtime.prepare_wasi_ctx(wasi_builder)?;

        let wasi = wasi_builder.build();
        let mut store = Store::new(&self.engine, wasi);

        linker.module(&mut store, "", &self.module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), ()>(&store)?
            .call(&mut store, ())?;

        drop(store);

        let err_contents: Vec<u8> = stderr
            .try_into_inner()
            .map_err(|_err| anyhow::Error::msg("Nothing to show"))?
            .into_inner();

        let string_err = String::from_utf8(err_contents)?;
        if !string_err.is_empty() {
            println!("Error: {string_err}");
        }

        let contents: Vec<u8> = stdout
            .try_into_inner()
            .map_err(|_err| anyhow::Error::msg("Nothing to show"))?
            .into_inner();

        // Build the output
        let output: WasmOutput = serde_json::from_slice(&contents)?;

        Ok(output)
    }
}
