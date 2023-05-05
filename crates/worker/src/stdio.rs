use std::{io::Cursor, fs::{File}};
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime_wasi::WasiCtxBuilder;

/// A library to configure the stdio of the WASI context.
/// Note that currently, wws relies on stdin and stdout
/// to send and read data from the worker.
///
/// The stderr is configurable just to cover use cases in which
/// wws is used as a library and we want to expose the logs
///
/// @see https://github.com/vmware-labs/wasm-workers-server/issues/125
///
/// The stdin/stdout approach will change in the future with
/// a more performant and appropiate way.
pub struct Stdio {
    /// Defines the stdin ReadPipe to send the data to the module
    pub stdin: ReadPipe<Cursor<String>>,
    /// Defines the stdout to extract the data from the module
    pub stdout: WritePipe<Cursor<Vec<u8>>>,
    /// Defines the stderr to expose logs from inside the module
    pub stderr: Option<WritePipe<File>>
}

impl Stdio {
    /// Initialize the stdio. The stdin will contain the input data.
    pub fn new(input: &str, stderr_file: Option<File>) -> Self {
        let stderr;

        if let Some(file) = stderr_file {
            stderr = Some(WritePipe::new(file));
        } else {
            stderr = None
        }

        Self {
            stdin: ReadPipe::from(input),
            stdout: WritePipe::new_in_memory(),
            stderr
        }
    }
    pub fn configure_wasi_ctx(&self, builder: WasiCtxBuilder) -> WasiCtxBuilder {
        let builder = builder
            .stdin(Box::new(self.stdin.clone()))
            .stdout(Box::new(self.stdout.clone()));

        if let Some(pipe) = self.stderr.clone() {
            builder.stderr(Box::new(pipe))
        } else {
            builder.inherit_stderr()
        }
    }
}
