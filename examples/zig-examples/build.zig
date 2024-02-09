const std = @import("std");
const wws = @import("wws");

pub fn build(b: *std.Build) !void {
    const target = wws.getTarget(b);
    const optimize = b.standardOptimizeOption(.{});

    const wws_dep = b.dependency("wws", .{});

    const wf = b.addWriteFiles();

    {
        const worker = try wws.addWorker(b, .{
            .name = "example",
            .path = "example",
            .root_source_file = .{ .path = "src/main.zig" },
            .target = target,
            .optimize = optimize,
            .wws = wws_dep,
            .features = .{
                .kv = .{
                    .namespace = "example",
                },
            },
        });

        try worker.addToWriteFiles(b, wf);
    }

    {
        const worker = try wws.addWorker(b, .{
            .name = "echo",
            .path = "echo",
            .root_source_file = .{ .path = "src/echo.zig" },
            .target = target,
            .optimize = optimize,
            .wws = wws_dep,
        });
        try worker.addToWriteFiles(b, wf);
    }

    const install = b.addInstallDirectory(.{
        .source_dir = wf.getDirectory(),
        .install_dir = .prefix,
        .install_subdir = "root",
    });

    b.getInstallStep().dependOn(&install.step);
}
