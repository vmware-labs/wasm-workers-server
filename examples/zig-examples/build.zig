const std = @import("std");
const wws = @import("wws");

const examples = &[_]Example{
    .{
        .name = "basic",
        .root_source_file = .{ .path = "src/basic.zig" },
    },
    .{
        .name = "envs",
        .root_source_file = .{ .path = "src/envs.zig" },
        .features = .{
            .vars = &.{
                .{ .name = "MESSAGE", .value = "Hello! This message comes from an environment variable" },
            },
        },
    },
    .{
        .name = "workerkv",
        .root_source_file = .{ .path = "src/worker-kv.zig" },
        .features = .{ .kv = .{ .namespace = "workerkv" } },
    },
    .{
        .name = "no-alloc-kv",
        .root_source_file = .{ .path = "src/no-alloc-kv.zig" },
        .features = .{ .kv = .{ .namespace = "workerkv" } },
    },
    .{
        .name = "mixed-alloc-kv",
        .root_source_file = .{ .path = "src/mixed-alloc-kv.zig" },
        .features = .{ .kv = .{ .namespace = "workerkv" } },
    },
    .{
        .name = "mount",
        .root_source_file = .{ .path = "src/mount.zig" },
        .features = .{
            .folders = &.{
                .{
                    .from = "./_images",
                    .to = "/src/images",
                },
            },
        },
    },
    .{
        .name = "params",
        .root_source_file = .{ .path = "src/params.zig" },
        .path = "params/[id]",
    },
    .{
        .name = "router",
        .root_source_file = .{ .path = "src/router.zig" },
        .path = "router/[...path]",
    },
};

const Example = struct {
    name: []const u8,
    root_source_file: std.Build.LazyPath,
    path: ?[]const u8 = null,
    features: ?wws.Features = null,
};

pub fn build(b: *std.Build) !void {
    const target = wws.getTarget(b);
    const optimize = b.standardOptimizeOption(.{});

    const wws_dep = b.dependency("wws", .{});
    const zig_router_dep = b.dependency("zig-router", .{});

    const wf = b.addWriteFiles();

    inline for (examples) |e| {
        const worker = try wws.addWorker(b, .{
            .name = e.name,
            .path = e.path orelse e.name,
            .root_source_file = e.root_source_file,
            .target = target,
            .optimize = optimize,
            .wws = wws_dep,
            .features = e.features orelse .{},
        });

        worker.exe.root_module.addImport("zig-router", zig_router_dep.module("zig-router"));

        try worker.addToWriteFiles(b, wf);
    }

    // Add folder for mount example
    _ = wf.addCopyFile(.{ .path = "src/_images/zig.svg" }, "_images/zig.svg");

    const install = b.addInstallDirectory(.{
        .source_dir = wf.getDirectory(),
        .install_dir = .prefix,
        .install_subdir = "root",
    });

    b.getInstallStep().dependOn(&install.step);
}
