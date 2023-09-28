use std::io::Cursor;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime_wasi::preview2;
use wws_runtimes::CtxBuilder;

// WASI's Preview2 MemoryOutputPipe, used to retrieve the result from
// the worker, requires a `capacity` setting
// (https://docs.rs/wasmtime-wasi/13.0.0/wasmtime_wasi/preview2/pipe/struct.MemoryOutputPipe.html#method.new). We
// might look into a solution that is less restrictive than this. This
// effectively caps the output size a worker can produce.
const MAX_OUTPUT_BYTES: usize = 10240;

/// A library to configure the stdio of the WASI context.
/// Note that currently, wws relies on stdin and stdout
/// to send and read data from the worker.
///
/// The stdin/stdout approach will change in the future with
/// a more performant and appropiate approach.
pub struct Stdio {
    /// Defines the stdin ReadPipe to send data to the module
    pub stdin: Vec<u8>,
    /// Defines the stdout to extract data from the module
    pub stdout: WritePipe<Cursor<Vec<u8>>>,
    /// Defines the stdout to extract data from the module
    pub stdout_preview2: preview2::pipe::MemoryOutputPipe,
}

impl Stdio {
    /// Initialize the stdio. The stdin will contain the input data.
    pub fn new(input: &str) -> Self {
        Self {
            stdin: Vec::from(input),
            stdout: WritePipe::new_in_memory(),
            stdout_preview2: preview2::pipe::MemoryOutputPipe::new(MAX_OUTPUT_BYTES),
        }
    }

    pub fn configure_wasi_ctx(&self, mut builder: CtxBuilder) -> CtxBuilder {
        match builder {
            CtxBuilder::Preview1(ref mut wasi_builder) => {
                wasi_builder
                    .stdin(Box::new(ReadPipe::from(self.stdin.clone()).clone()))
                    .stdout(Box::new(self.stdout.clone()))
                    .inherit_stderr();
            }
            CtxBuilder::Preview2(ref mut wasi_builder) => {
                wasi_builder
                    .stdin(
                        preview2::pipe::MemoryInputPipe::new(self.stdin.clone().into()),
                        preview2::IsATTY::No,
                    )
                    .stdout(self.stdout_preview2.clone(), preview2::IsATTY::No)
                    .inherit_stderr();
            }
        }
        builder
    }
}
