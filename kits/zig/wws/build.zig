const std = @import("std");

pub fn build(b: *std.Build) void {
    _ = b.standardTargetOptions(.{});
    _ = b.standardOptimizeOption(.{});

    const module = b.addModule("wws", .{
        .root_source_file = .{ .path = "src/wws.zig" },
        .target = getTarget(b),
    });

    _ = module;
}

pub inline fn getTarget(b: *std.Build) std.Build.ResolvedTarget {
    return b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .wasi,
    });
}

pub const WwsLibOptions = struct {
    name: []const u8,
    root_source_file: std.Build.LazyPath,
    optimize: std.builtin.OptimizeMode,
    imports: []const std.Build.Module.Import = &.{},
};

pub fn addExecutable(b: *std.Build, options: WwsLibOptions) *std.Build.Step.Compile {
    const exe = b.addExecutable(.{
        .name = options.name,
        .root_source_file = options.root_source_file,
        .target = getTarget(b),
        .optimize = options.optimize,
    });

    exe.wasi_exec_model = .reactor;

    for (options.imports) |import| {
        exe.root_module.addImport(import.name, import.module);
    }

    return exe;
}
