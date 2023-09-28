// Copyright 2022-2023 VMware, Inc.
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
use io::{WasmInput, WasmOutput};
use sha256::digest as sha256_digest;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, path::Path};
use stdio::Stdio;
use wasi_common::WasiCtx;
use wasmtime::{component::Component, Engine, Linker, Module, Store};
use wasmtime_wasi::{ambient_authority, Dir, WasiCtxBuilder};
use wasmtime_wasi_nn::{InMemoryRegistry, Registry, WasiNnCtx};
use wws_config::Config as ProjectConfig;
use wws_runtimes::{init_runtime, Runtime};

pub enum ModuleOrComponent {
    Module(Module),
    Component(Component),
}

/// A worker contains the engine and the associated runtime.
/// This struct will process requests by preparing the environment
/// with the runtime and running it in Wasmtime
pub struct Worker {
    /// Worker identifier
    pub id: String,
    /// Wasmtime engine to run this worker
    engine: Engine,
    /// Wasm Module or component
    module_or_component: ModuleOrComponent,
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
        let module_or_component = if let Ok(module) = Module::from_binary(&engine, &bytes) {
            Ok(ModuleOrComponent::Module(module))
        } else if let Ok(component) = Component::from_binary(&engine, &bytes) {
            Ok(ModuleOrComponent::Component(component))
        } else {
            Err(errors::WorkerError::BadWasmModuleOrComponent)
        }?;

        // Prepare the environment if required
        runtime.prepare()?;

        Ok(Self {
            id,
            engine,
            module_or_component,
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

        http_add_to_linker(&mut linker, |s: &mut WorkerState| &mut s.http).map_err(|error| {
            errors::WorkerError::ConfigureRuntimeError {
                error: format!("error adding HTTP bindings to linker ({error})"),
            }
        })?;
        wasmtime_wasi::add_to_linker(&mut linker, |s: &mut WorkerState| &mut s.wasi).map_err(
            |error| errors::WorkerError::ConfigureRuntimeError {
                error: format!("error adding WASI to linker ({error})"),
            },
        )?;

        // I have to use `String` as it's required by WasiCtxBuilder
        let tuple_vars: Vec<(String, String)> =
            vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        // Create the initial WASI context
        let mut wasi_builder = WasiCtxBuilder::new();
        wasi_builder.envs(&tuple_vars).map_err(|error| {
            errors::WorkerError::ConfigureRuntimeError {
                error: format!("error configuring runtime: {error}"),
            }
        })?;

        // Configure the stdio
        let stdio = Stdio::new(&input);
        stdio.configure_wasi_ctx(&mut wasi_builder);

        // Mount folders from the configuration
        if let Some(folders) = self.config.folders.as_ref() {
            for folder in folders {
                if let Some(base) = &self.path.parent() {
                    let dir = Dir::open_ambient_dir(base.join(&folder.from), ambient_authority())
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                        error: format!("error setting up pre-opened folders: {error}"),
                    })?;
                    wasi_builder
                        .preopened_dir(dir, &folder.to)
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error setting up pre-opened folders: {error}"),
                        })?;
                } else {
                    return Err(errors::WorkerError::FailedToInitialize);
                }
            }
        }

        // WASI-NN
        let allowed_backends = &self.config.features.wasi_nn.allowed_backends;
        let preload_models = &self.config.features.wasi_nn.preload_models;

        let wasi_nn = if !preload_models.is_empty() {
            // Preload the models on the host.
            let graphs = preload_models
                .iter()
                .map(|m| m.build_graph_data(&self.path))
                .collect::<Vec<_>>();
            let (backends, registry) = wasmtime_wasi_nn::preload(&graphs).map_err(|_| {
                errors::WorkerError::RuntimeError(
                    wws_runtimes::errors::RuntimeError::WasiContextError,
                )
            })?;

            Some(Arc::new(WasiNnCtx::new(backends, registry)))
        } else if !allowed_backends.is_empty() {
            let registry = Registry::from(InMemoryRegistry::new());
            let mut backends = Vec::new();

            // Load the given backends:
            for b in allowed_backends.iter() {
                if let Some(backend) = b.to_backend() {
                    backends.push(backend);
                }
            }

            Some(Arc::new(WasiNnCtx::new(backends, registry)))
        } else {
            None
        };

        // Load the Wasi NN linker
        if wasi_nn.is_some() {
            wasmtime_wasi_nn::witx::add_to_linker(&mut linker, |s: &mut WorkerState| {
                Arc::get_mut(s.wasi_nn.as_mut().unwrap())
                    .expect("wasi-nn is not implemented with multi-threading support")
            })
            .map_err(|_| {
                errors::WorkerError::RuntimeError(
                    wws_runtimes::errors::RuntimeError::WasiContextError,
                )
            })?;
        }

        // Pass to the runtime to add any WASI specific requirement
        self.runtime.prepare_wasi_ctx(&mut wasi_builder)?;

        let wasi = wasi_builder.build();
        let state = WorkerState {
            wasi,
            wasi_nn,
            http: HttpBindings {
                http_config: self.config.features.http_requests.clone(),
            },
        };
        let mut store = Store::new(&self.engine, state);

        let module = match &self.module_or_component {
            ModuleOrComponent::Module(module) => module,
            ModuleOrComponent::Component(_) => unimplemented!(),
        };

        linker.module(&mut store, "", &module).map_err(|error| {
            errors::WorkerError::ConfigureRuntimeError {
                error: format!("error retrieving module from linker: {error}"),
            }
        })?;
        linker
            .get_default(&mut store, "")
            .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                error: format!("error getting default export from module: {error}"),
            })?
            .typed::<(), ()>(&store)
            .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                error: format!("error getting default typed export from module: {error}"),
            })?
            .call(&mut store, ())
            .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                error: format!("error calling module default export: {error}"),
            })?;

        drop(store);

        let contents: Vec<u8> = stdio
            .stdout
            .try_into_inner()
            .unwrap_or_default()
            .into_inner();

        // Build the output
        let output: WasmOutput = serde_json::from_slice(&contents).map_err(|error| {
            errors::WorkerError::ConfigureRuntimeError {
                error: format!("error building worker output: {error}"),
            }
        })?;

        Ok(output)
    }
}
