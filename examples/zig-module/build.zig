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
