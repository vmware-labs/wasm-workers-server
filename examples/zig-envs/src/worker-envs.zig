const std = @import("std");
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

fn requestFn(resp: *worker.Response, r: *worker.Request) void {
    _ = r;
    var m: []const u8 = "none";

    if (std.os.getenv("MESSAGE")) |env| {
        std.debug.print("getenv: {s}", .{ env });
        m = env;
    }

    const s =
        \\The environment variable value is: {s}
    ;

    var body = std.fmt.allocPrint(allocator, s, .{ m }) catch undefined; // add useragent

    _ = &resp.headers.append("x-generated-by", "wasm-workers-server");
    _ = &resp.writeAll(body);
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
