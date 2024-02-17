const std = @import("std");

const module_name = "wws";

pub fn build(b: *std.Build) void {
    _ = b.standardTargetOptions(.{});
    _ = b.standardOptimizeOption(.{});

    const module = b.addModule(module_name, .{
        .root_source_file = .{ .path = "src/wws.zig" },
        .target = getTarget(b),
    });

    _ = module;
}

// Returns the wasm32-wasi target
pub inline fn getTarget(b: *std.Build) std.Build.ResolvedTarget {
    return b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .wasi,
    });
}

pub const EnvVar = struct {
    name: []const u8,
    value: []const u8,
};

pub const WwsKv = struct {
    namespace: []const u8,
};

pub const Mount = struct {
    from: []const u8,
    to: []const u8,
};

pub const Features = struct {
    vars: ?[]const EnvVar = null,
    kv: ?WwsKv = null,
    folders: ?[]const Mount = null,
};

pub const WwsWorker = struct {
    exe: *std.Build.Step.Compile,
    options: WorkerOptions,

    pub fn addToWriteFiles(self: WwsWorker, b: *std.Build, wf: *std.Build.Step.WriteFile) !void {
        _ = wf.addCopyFile(self.exe.getEmittedBin(), b.fmt("{s}.wasm", .{self.options.path}));

        const config = try self.formatConfig(b.allocator);
        defer b.allocator.free(config);
        _ = wf.add(b.fmt("{s}.toml", .{self.options.path}), config);
    }

    // Very naively write the worker config to a string. Environment variable values
    // and KV namespace name are automatically quoted.
    pub fn formatConfig(self: WwsWorker, allocator: std.mem.Allocator) ![]u8 {
        var buf = std.ArrayList(u8).init(allocator);
        errdefer buf.deinit();

        try buf.writer().print(
            \\name = "{s}"
            \\version = "1"
            \\
        ,
            .{
                self.options.name,
            },
        );

        if (self.options.features.vars) |vars| {
            try buf.writer().print(
                \\[vars]
                \\
            ,
                .{},
            );
            for (vars) |v| {
                try buf.writer().print(
                    \\{s} = "{s}"
                    \\
                ,
                    .{ v.name, v.value },
                );
            }
        }

        if (self.options.features.kv) |kv| {
            try buf.writer().print(
                \\[data]
                \\[data.kv]
                \\namespace = "{s}"
                \\
            ,
                .{
                    kv.namespace,
                },
            );
        }

        if (self.options.features.folders) |folders| {
            try buf.writer().print(
                \\[[folders]]
                \\
            ,
                .{},
            );

            for (folders) |f| {
                try buf.writer().print(
                    \\from = "{s}"
                    \\to = "{s}"
                    \\
                ,
                    .{
                        f.from,
                        f.to,
                    },
                );
            }
        }

        return try buf.toOwnedSlice();
    }
};

pub const WorkerOptions = struct {
    // The name of the module, written to the worker config
    name: []const u8,
    root_source_file: std.Build.LazyPath,
    // The relative path of the module and its worker config.
    // E.g. `.{ .path = "api/[...params]" }` will write to
    // `api/[...params].wasm` and `api/[...params].toml`.
    path: []const u8,
    // The worker features/config including KV namespace,
    // environment variables, and more. These are written
    // to the worker config.
    features: Features = .{},
    // You must provide the wws dependency to ensure the
    // wws module is linked to your project.
    wws: *std.Build.Dependency,
    target: std.Build.ResolvedTarget,
    optimize: std.builtin.OptimizeMode,
};

pub fn addWorker(b: *std.Build, options: WorkerOptions) !WwsWorker {
    const exe = b.addExecutable(.{
        .name = options.name,
        .root_source_file = options.root_source_file,
        .target = options.target,
        .optimize = options.optimize,
    });

    exe.wasi_exec_model = .reactor;
    exe.root_module.addImport("wws", options.wws.module(module_name));

    return .{
        .exe = exe,
        .options = options,
    };
}
