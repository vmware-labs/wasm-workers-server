use std::io::Cursor;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime_wasi::WasiCtxBuilder;

/// A library to configure the stdio of the WASI context.
/// Note that currently, wws relies on stdin and stdout
/// to send and read data from the worker.
///
/// The stdin/stdout approach will change in the future with
/// a more performant and appropiate approach.
pub struct Stdio {
    /// Defines the stdin ReadPipe to send the data to the module
    pub stdin: ReadPipe<Cursor<String>>,
    /// Defines the stdout to extract the data from the module
    pub stdout: WritePipe<Cursor<Vec<u8>>>,
}

impl Stdio {
    /// Initialize the stdio. The stdin will contain the input data.
    pub fn new(input: &str) -> Self {
        Self {
            stdin: ReadPipe::from(input),
            stdout: WritePipe::new_in_memory(),
        }
    }

    pub fn configure_wasi_ctx(&self, builder: WasiCtxBuilder) -> WasiCtxBuilder {
        builder
            .stdin(Box::new(self.stdin.clone()))
            .stdout(Box::new(self.stdout.clone()))
            .inherit_stderr()
    }
}
