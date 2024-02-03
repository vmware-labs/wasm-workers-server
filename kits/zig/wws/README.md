# Zig kit

This folder contains the Zig kit or SDK for Wasm Workers Server. Currently, it uses the regular STDIN / STDOUT approach to receive the request and provide the response.

> *Note: Last tested with Zig `0.12.0-dev.2208+4debd4338`*

## Usage

Add wws as a dependency to your Zig project:

```shell-session
$ zig fetch --save <path_to_wws>
```

Include it in your build.zig:

```zig
const std = @import("std");
const wws = @import("wws");

pub fn build(b: *std.Build) !void {
    const target = wws.getTarget(b);
    const optimize = b.standardOptimizeOption(.{});

    const wws_dep = b.dependency("wws", .{});

    const exe = b.addExecutable(.{
        .name = "example",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });
    exe.wasi_exec_model = .reactor;
    exe.root_module.addImport("wws", wws_dep.module("wws"));

    b.installArtifact(exe);

    const config =
        \\name = "example"
        \\version = "1"
        \\[data]
        \\[data.kv]
        \\namespace = "example"
    ;
    const wf = b.addWriteFiles();
    const config_path = wf.add("example.toml", config);

    const install_config = b.addInstallBinFile(config_path, "example.toml");

    b.getInstallStep().dependOn(&install_config.step);
}
```

Read request from stdin and write response to stdout:

```zig
const std=@import("std");
const wws=@import("wws");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        switch (gpa.deinit()) {
            .ok => {},
            .leak => {
                // Handle memory leaks however you want (e.g. error logging).
                // Note that if your main function returns an error,
                // wws will not return the response you've crafted.
            },
        }
    }
    const allocator = gpa.allocator();

    // Read request from stdin
    const parse_result = try wws.parseStream(allocator, .{});
    defer parse_result.deinit();

    const request = parse_result.value;

    // Simple echo response
    const response = wws.Response{
        .data = request.url,
    };

    const stdout = std.io.getStdOut();
    try wws.writeResponse(response, stdout.writer());
}
```

## Build

To build all examples in ./examples/zig-examples

```shell-session
$ zig build all
```

To build a specific example, for example kv:

```shell-session
$ zig build kv
```

## Run

This assumes you have `wws` available in your PATH:

```shell-session
$ zig build run
```

## Testing

At the Zig SDK path, execute:

```shell-session
$ zig build test
```

