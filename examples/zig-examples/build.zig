const std = @import("std");
const wws = @import("wws");

const Example = struct {
    name: []const u8,
    root_source_file: std.Build.LazyPath,
    path: ?[]const u8 = null,
    features: ?wws.Features = null,
};

const examples = &[_]Example{
    .{
        .name = "example",
        .root_source_file = .{ .path = "src/main.zig" },
        .features = .{
            .kv = .{
                .namespace = "example",
            },
        },
    },
    .{
        .name = "echo",
        .root_source_file = .{ .path = "src/echo.zig" },
    },
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
        .path = "[...params]",
    },
};

pub fn build(b: *std.Build) !void {
    const target = wws.getTarget(b);
    const optimize = b.standardOptimizeOption(.{});

    const wws_dep = b.dependency("wws", .{});

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

        try worker.addToWriteFiles(b, wf);
    }

    const install = b.addInstallDirectory(.{
        .source_dir = wf.getDirectory(),
        .install_dir = .prefix,
        .install_subdir = "root",
    });

    b.getInstallStep().dependOn(&install.step);
}
