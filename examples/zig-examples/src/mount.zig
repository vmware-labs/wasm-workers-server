const std = @import("std");
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

fn requestFn(resp: *worker.Response, r: *worker.Request) void {
    _ = r;

    const file = std.fs.Dir.openFileWasi(
        std.fs.cwd(), "zig.svg", .{
            .mode = std.fs.File.OpenMode.read_only,
            .lock = std.fs.File.Lock.none,
        }) catch unreachable;
    defer file.close();

    const mb = (1 << 10) << 10;
    const file_contents = file.readToEndAlloc(allocator, mb) catch "";

    _ = &resp.headers.append("x-generated-by", "wasm-workers-server");
    _ = &resp.writeAll(file_contents);
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
