// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

mod bindings;
pub mod config;
pub mod errors;
pub mod features;
pub mod io;
mod stdio;

use actix_web::HttpRequest;
use bindings::http::{add_to_linker as http_add_to_linker, HttpBindings};
use config::Config;
use errors::Result;
use features::wasi_nn::WASI_NN_BACKEND_OPENVINO;
use io::{WasmInput, WasmOutput};
use sha256::digest as sha256_digest;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, path::Path};
use stdio::Stdio;
use wasi_common::WasiCtx;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{ambient_authority, Dir, WasiCtxBuilder};
use wasmtime_wasi_nn::WasiNnCtx;
use wws_config::Config as ProjectConfig;
use wws_runtimes::{init_runtime, Runtime};

/// A worker contains the engine and the associated runtime.
/// This struct will process requests by preparing the environment
/// with the runtime and running it in Wasmtime
pub struct Worker {
    /// Worker identifier
    pub id: String,
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

struct WorkerState {
    pub wasi: WasiCtx,
    pub wasi_nn: Option<Arc<WasiNnCtx>>,
    pub http: HttpBindings,
}

impl Worker {
    /// Creates a new Worker
    pub fn new(project_root: &Path, path: &Path, project_config: &ProjectConfig) -> Result<Self> {
        // Compute the identifier
        let id = sha256_digest(project_root.join(path).to_string_lossy().as_bytes());

        // Load configuration
        let mut config_path = path.to_path_buf();
        config_path.set_extension("toml");
        let mut config = Config::default();

        if fs::metadata(&config_path).is_ok() {
            match Config::try_from_file(config_path) {
                Ok(c) => config = c,
                Err(e) => {
                    eprintln!("Error loading the worker configuration: {}", e);
                }
            }
        }

        let engine = Engine::default();
        let runtime = init_runtime(project_root, path, project_config)?;
        let bytes = runtime.module_bytes()?;
        let module =
            Module::from_binary(&engine, &bytes).map_err(|_| errors::WorkerError::BadWasmModule)?;

        // Prepare the environment if required
        runtime.prepare()?;

        Ok(Self {
            id,
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

        let mut linker = Linker::new(&self.engine);

        http_add_to_linker(&mut linker, |s: &mut WorkerState| &mut s.http)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;
        wasmtime_wasi::add_to_linker(&mut linker, |s: &mut WorkerState| &mut s.wasi)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;

        // I have to use `String` as it's required by WasiCtxBuilder
        let tuple_vars: Vec<(String, String)> =
            vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        // Create the initial WASI context
        let mut wasi_builder = WasiCtxBuilder::new()
            .envs(&tuple_vars)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;

        // Configure the stdio
        let stdio = Stdio::new(&input);
        wasi_builder = stdio.configure_wasi_ctx(wasi_builder);

        // Mount folders from the configuration
        if let Some(folders) = self.config.folders.as_ref() {
            for folder in folders {
                if let Some(base) = &self.path.parent() {
                    let dir = Dir::open_ambient_dir(base.join(&folder.from), ambient_authority())
                        .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;
                    wasi_builder = wasi_builder
                        .preopened_dir(dir, &folder.to)
                        .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;
                } else {
                    return Err(errors::WorkerError::FailedToInitialize);
                }
            }
        }

        // WASI-NN
        let allowed_backends = &self.config.features.wasi_nn.allowed_backends;

        let wasi_nn = if !allowed_backends.is_empty() {
            // For now, we only support OpenVINO
            if allowed_backends.len() != 1
                || !allowed_backends.contains(&WASI_NN_BACKEND_OPENVINO.to_string())
            {
                eprintln!("❌ The only WASI-NN supported backend name is \"{WASI_NN_BACKEND_OPENVINO}\". Please, update your config.");
                None
            } else {
                wasmtime_wasi_nn::add_to_linker(&mut linker, |s: &mut WorkerState| {
                    Arc::get_mut(s.wasi_nn.as_mut().unwrap())
                        .expect("wasi-nn is not implemented with multi-threading support")
                })
                .map_err(|_| {
                    errors::WorkerError::RuntimeError(
                        wws_runtimes::errors::RuntimeError::WasiContextError,
                    )
                })?;

                Some(Arc::new(WasiNnCtx::new().map_err(|_| {
                    errors::WorkerError::RuntimeError(
                        wws_runtimes::errors::RuntimeError::WasiContextError,
                    )
                })?))
            }
        } else {
            None
        };

        // Pass to the runtime to add any WASI specific requirement
        wasi_builder = self.runtime.prepare_wasi_ctx(wasi_builder)?;

        let wasi = wasi_builder.build();
        let state = WorkerState {
            wasi,
            wasi_nn,
            http: HttpBindings {
                http_config: self.config.features.http_requests.clone(),
            },
        };
        let mut store = Store::new(&self.engine, state);

        linker
            .module(&mut store, "", &self.module)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;
        linker
            .get_default(&mut store, "")
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?
            .typed::<(), ()>(&store)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?
            .call(&mut store, ())
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;

        drop(store);

        let contents: Vec<u8> = stdio
            .stdout
            .try_into_inner()
            .unwrap_or_default()
            .into_inner();

        // Build the output
        let output: WasmOutput = serde_json::from_slice(&contents)
            .map_err(|_| errors::WorkerError::ConfigureRuntimeError)?;

        Ok(output)
    }
}
