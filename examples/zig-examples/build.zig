const std = @import("std");
const wws = @import("wws");

const WwsKv = struct {
    namespace: []const u8,
};

const WwsFeatures = struct {
    kv: ?WwsKv = null,
};

const ModuleOptions = struct {
    name: []const u8,
    target: std.Build.ResolvedTarget,
    optimize: std.builtin.OptimizeMode,
    features: WwsFeatures = .{},
    wws: *std.Build.Dependency,
    root_source_file: std.Build.LazyPath,
};

fn addModule(b: *std.Build, options: ModuleOptions) !void {
    const exe = b.addExecutable(.{
        .name = options.name,
        .root_source_file = options.root_source_file,
        .target = options.target,
        .optimize = options.optimize,
    });
    exe.wasi_exec_model = .reactor;
    exe.root_module.addImport("wws", options.wws.module("wws"));

    b.installArtifact(exe);

    var buf = std.ArrayList(u8).init(b.allocator);
    defer buf.deinit();

    try buf.writer().print(
        \\name = "{s}"
        \\version = "1"
        ,
        .{
            options.name,
        },
    );

    if (options.features.kv) |kv| {

        try buf.writer().print(
            \\[data]
            \\[data.kv]
            \\namespace = "{s}"
            ,
            .{
                kv.namespace,
            },
        );
    }

    const wf = b.addWriteFiles();
    const config_name = b.fmt("{s}.toml", .{ options.name});
    const config_path = wf.add(config_name, try buf.toOwnedSlice(),);

    const install_config = b.addInstallBinFile(config_path, config_name);

    b.getInstallStep().dependOn(&install_config.step);
}

pub fn build(b: *std.Build) !void {
    const target = wws.getTarget(b);
    const optimize = b.standardOptimizeOption(.{});

    const wws_dep = b.dependency("wws", .{});

    try addModule(b, .{
        .name = "example",
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

    try addModule(b, .{
        .name = "echo",
        .root_source_file = .{ .path = "src/echo.zig" },
        .target = target,
        .optimize = optimize,
        .wws = wws_dep,
    });
}
