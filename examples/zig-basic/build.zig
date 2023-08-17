const std = @import("std");

const examples = [1][]const u8{ "basic" };

pub fn build(b: *std.Build) !void {
    const target = try std.zig.CrossTarget.parse(.{ .arch_os_abi = "wasm32-wasi" });
    const optimize = b.standardOptimizeOption(.{});

    const worker_module = b.createModule(.{
        .source_file = .{ .path = "../../kits/zig/worker/src/worker.zig" },
    });

    inline for (examples) |example| {
        const exe = b.addExecutable(.{
            .name = example,
            .root_source_file = .{ .path = "src/" ++ example ++ ".zig" },
            .target = target,
            .optimize = optimize,
        });

        exe.wasi_exec_model = .reactor;
        exe.addModule("worker", worker_module);

        b.installArtifact(exe);
    }
}
