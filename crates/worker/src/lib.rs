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
use wasmtime::{
    component::{self, Component},
    Config as WasmtimeConfig, Engine, Linker, Module, Store,
};
use wasmtime_wasi::{ambient_authority, preview2, Dir, WasiCtxBuilder};
use wasmtime_wasi_nn::{InMemoryRegistry, Registry, WasiNnCtx};
use wws_config::Config as ProjectConfig;
use wws_runtimes::{init_runtime, CtxBuilder, Runtime};

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
    /// Worker runtime
    runtime: Box<dyn Runtime + Sync + Send>,
    /// Wasm Module or component
    module_or_component: ModuleOrComponent,
    /// Current config
    pub config: Config,
    /// The worker filepath
    path: PathBuf,
}

#[derive(Default)]
struct Host {
    pub wasi_preview1_ctx: Option<wasmtime_wasi::WasiCtx>,
    pub wasi_preview2_ctx: Option<Arc<preview2::WasiCtx>>,

    // Resource table for preview2 if the `preview2_ctx` is in use, otherwise
    // "just" an empty table.
    wasi_preview2_table: Arc<preview2::Table>,

    // State necessary for the preview1 implementation of WASI backed by the
    // preview2 host implementation. Only used with the `--preview2` flag right
    // now when running core modules.
    wasi_preview2_adapter: Arc<preview2::preview1::WasiPreview1Adapter>,

    pub wasi_nn: Option<Arc<WasiNnCtx>>,
    pub http: Option<HttpBindings>,
}

impl preview2::WasiView for Host {
    fn table(&self) -> &preview2::Table {
        &self.wasi_preview2_table
    }

    fn table_mut(&mut self) -> &mut preview2::Table {
        Arc::get_mut(&mut self.wasi_preview2_table)
            .expect("preview2 is not compatible with threads")
    }

    fn ctx(&self) -> &preview2::WasiCtx {
        self.wasi_preview2_ctx.as_ref().unwrap()
    }

    fn ctx_mut(&mut self) -> &mut preview2::WasiCtx {
        let ctx = self.wasi_preview2_ctx.as_mut().unwrap();
        Arc::get_mut(ctx).expect("preview2 is not compatible with threads")
    }
}

impl preview2::preview1::WasiPreview1View for Host {
    fn adapter(&self) -> &preview2::preview1::WasiPreview1Adapter {
        &self.wasi_preview2_adapter
    }

    fn adapter_mut(&mut self) -> &mut preview2::preview1::WasiPreview1Adapter {
        Arc::get_mut(&mut self.wasi_preview2_adapter)
            .expect("preview2 is not compatible with threads")
    }
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

        let engine = Engine::new(
            WasmtimeConfig::default()
                .async_support(true)
                .wasm_component_model(true),
        )
        .map_err(|err| errors::WorkerError::ConfigureRuntimeError {
            error: format!("error creating engine ({err})"),
        })?;
        let runtime = init_runtime(project_root, path, project_config)?;
        let bytes = runtime.module_bytes()?;

        let module_or_component = if wasmparser::Parser::is_core_wasm(&bytes) {
            Ok(ModuleOrComponent::Module(
                Module::from_binary(&engine, &bytes).map_err(|err| {
                    errors::WorkerError::BadWasmCoreModule {
                        error: format!("{:?}", err),
                    }
                })?,
            ))
        } else if wasmparser::Parser::is_component(&bytes) {
            Ok(ModuleOrComponent::Component(
                Component::from_binary(&engine, &bytes).map_err(|err| {
                    errors::WorkerError::BadWasmComponent {
                        error: format!("{:?}", err),
                    }
                })?,
            ))
        } else {
            Err(errors::WorkerError::BadWasmCoreModuleOrComponent)
        }?;

        // Prepare the environment if required
        runtime.prepare()?;

        Ok(Self {
            id,
            engine,
            runtime,
            module_or_component,
            config,
            path: path.to_path_buf(),
        })
    }

    pub fn prepare_wasi_context(
        &self,
        environment_variables: &[(String, String)],
        wasi_builder: &mut CtxBuilder,
    ) -> Result<()> {
        match wasi_builder {
            CtxBuilder::Preview1(wasi_builder) => {
                // Set up environment variables
                wasi_builder.envs(environment_variables).map_err(|error| {
                    errors::WorkerError::ConfigureRuntimeError {
                        error: format!("error configuring runtime: {error}"),
                    }
                })?;

                // Setup pre-opens
                if let Some(folders) = self.config.folders.as_ref() {
                    for folder in folders {
                        if let Some(base) = &self.path.parent() {
                            let dir =
                                Dir::open_ambient_dir(base.join(&folder.from), ambient_authority())
                                    .map_err(|error| {
                                        errors::WorkerError::ConfigureRuntimeError {
                                            error: format!(
                                                "error setting up pre-opened folders: {error}"
                                            ),
                                        }
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
            }
            CtxBuilder::Preview2(wasi_builder) => {
                // Set up environment variables
                wasi_builder.envs(environment_variables);

                // Setup pre-opens
                if let Some(folders) = self.config.folders.as_ref() {
                    for folder in folders {
                        if let Some(base) = &self.path.parent() {
                            let dir =
                                Dir::open_ambient_dir(base.join(&folder.from), ambient_authority())
                                    .map_err(|error| {
                                        errors::WorkerError::ConfigureRuntimeError {
                                            error: format!(
                                                "error setting up pre-opened folders: {error}"
                                            ),
                                        }
                                    })?;
                            wasi_builder.preopened_dir(
                                dir,
                                preview2::DirPerms::all(),
                                preview2::FilePerms::all(),
                                &folder.to,
                            );
                        } else {
                            return Err(errors::WorkerError::FailedToInitialize);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn run(
        &self,
        request: &HttpRequest,
        body: &str,
        kv: Option<HashMap<String, String>>,
        vars: &HashMap<String, String>,
    ) -> Result<WasmOutput> {
        let input = serde_json::to_string(&WasmInput::new(request, body, kv)).unwrap();

        let mut linker = Linker::new(&self.engine);
        let mut component_linker = component::Linker::new(&self.engine);

        if let ModuleOrComponent::Module(_) = &self.module_or_component {
            wasmtime_wasi::add_to_linker(&mut linker, |host: &mut Host| {
                host.wasi_preview1_ctx.as_mut().unwrap()
            })
            .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                error: format!("error adding WASI preview1 to linker ({error})"),
            })?;

            http_add_to_linker(&mut linker, |host: &mut Host| host.http.as_mut().unwrap())
                .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                    error: format!("error adding HTTP bindings to linker ({error})"),
                })?;
        } else {
            preview2::command::add_to_linker(&mut component_linker).map_err(|error| {
                errors::WorkerError::ConfigureRuntimeError {
                    error: format!("error adding WASI preview2 to linker ({error})"),
                }
            })?;
        }

        let environment_variables: Vec<(String, String)> =
            vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        let mut wasi_builder = if let ModuleOrComponent::Module(_) = &self.module_or_component {
            CtxBuilder::Preview1(WasiCtxBuilder::new())
        } else {
            CtxBuilder::Preview2(preview2::WasiCtxBuilder::new())
        };
        self.prepare_wasi_context(&environment_variables, &mut wasi_builder)?;

        let stdio = Stdio::new(&input);
        let mut wasi_builder = stdio.configure_wasi_ctx(wasi_builder);

        self.runtime.prepare_wasi_ctx(&mut wasi_builder)?;

        let allowed_backends = &self.config.features.wasi_nn.allowed_backends;
        let preload_models = &self.config.features.wasi_nn.preload_models;
        let wasi_nn = if !preload_models.is_empty() {
            // Preload the models on the host.
            let graphs = preload_models
                .iter()
                .map(|m| m.build_graph_data(&self.path))
                .collect::<Vec<_>>();
            let (backends, registry) = wasmtime_wasi_nn::preload(&graphs).map_err(|err| {
                errors::WorkerError::RuntimeError(
                    wws_runtimes::errors::RuntimeError::WasiContextError {
                        error: format!("{}", err),
                    },
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

        let host = match wasi_builder {
            CtxBuilder::Preview1(mut wasi_builder) => {
                if wasi_nn.is_some() {
                    wasmtime_wasi_nn::witx::add_to_linker(&mut linker, |host: &mut Host| {
                        Arc::get_mut(host.wasi_nn.as_mut().unwrap()).unwrap()
                    })
                    .map_err(|err| {
                        errors::WorkerError::RuntimeError(
                            wws_runtimes::errors::RuntimeError::WasiContextError {
                                error: format!("{}", err),
                            },
                        )
                    })?;
                }
                Host {
                    wasi_preview1_ctx: Some(wasi_builder.build()),
                    wasi_nn,
                    http: Some(HttpBindings {
                        http_config: self.config.features.http_requests.clone(),
                    }),
                    ..Host::default()
                }
            }
            CtxBuilder::Preview2(mut wasi_builder) => {
                if wasi_nn.is_some() {
                    wasmtime_wasi_nn::wit::ML::add_to_linker(
                        &mut component_linker,
                        |host: &mut Host| Arc::get_mut(host.wasi_nn.as_mut().unwrap()).unwrap(),
                    )
                    .map_err(|err| {
                        errors::WorkerError::RuntimeError(
                            wws_runtimes::errors::RuntimeError::WasiContextError {
                                error: format!("{}", err),
                            },
                        )
                    })?;
                }
                let mut table = preview2::Table::default();
                Host {
                    wasi_preview2_ctx: Some(Arc::new(wasi_builder.build(&mut table).map_err(
                        |error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error configuring WASI preview 2: {error}"),
                        },
                    )?)),
                    wasi_preview2_table: Arc::new(table),
                    wasi_preview2_adapter: Arc::new(
                        preview2::preview1::WasiPreview1Adapter::default(),
                    ),
                    wasi_nn,
                    http: Some(HttpBindings {
                        http_config: self.config.features.http_requests.clone(),
                    }),
                    ..Host::default()
                }
            }
        };

        let contents = {
            let mut store = Store::new(&self.engine, host);
            match &self.module_or_component {
                ModuleOrComponent::Module(module) => {
                    linker
                        .module_async(&mut store, "", module)
                        .await
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error retrieving module from linker: {error}"),
                        })?;

                    linker
                        .get_default(&mut store, "")
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error getting default export from module: {error}"),
                        })?
                        .typed::<(), ()>(&store)
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!(
                                "error getting default typed export from module: {error}"
                            ),
                        })?
                        .call_async(&mut store, ())
                        .await
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error calling module default export: {error}"),
                        })?;

                    drop(store);

                    stdio
                        .stdout
                        .try_into_inner()
                        .unwrap_or_default()
                        .into_inner()
                }
                ModuleOrComponent::Component(component) => {
                    let (command, _instance) = preview2::command::Command::instantiate_async(
                        &mut store,
                        component,
                        &component_linker,
                    )
                    .await
                    .map_err(|error| {
                        errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error instantiating component cli::run: {error}"),
                        }
                    })?;

                    let _ = command
                        .wasi_cli_run()
                        .call_run(&mut store)
                        .await
                        .map_err(|error| errors::WorkerError::ConfigureRuntimeError {
                            error: format!("error calling component cli::run: {error}"),
                        })?;

                    drop(store);

                    stdio.stdout_preview2.contents().to_vec()
                }
            }
        };

        // Build the output
        let output: WasmOutput = serde_json::from_slice(&contents).map_err(|error| {
            errors::WorkerError::ConfigureRuntimeError {
                error: format!("error building worker output: {error}"),
            }
        })?;

        Ok(output)
    }
}
