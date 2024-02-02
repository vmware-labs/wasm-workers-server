const std = @import("std");

const examples = [1][]const u8{"worker-kv"};

pub fn build(b: *std.Build) !void {
    const target = b.resolveTargetQuery(.{
        .cpu_arch = .wasm32,
        .os_tag = .wasi,
    });
    const optimize = b.standardOptimizeOption(.{});

    const worker_module = b.createModule(.{
        .root_source_file = .{ .path = "../../kits/zig/worker/src/worker.zig" },
    });

    inline for (examples) |example| {
        const exe = b.addExecutable(.{
            .name = example,
            .root_source_file = .{ .path = "src/" ++ example ++ ".zig" },
            .target = target,
            .optimize = optimize,
        });

        exe.wasi_exec_model = .reactor;
        exe.root_module.addImport("worker", worker_module);

        b.installArtifact(exe);
    }
}
