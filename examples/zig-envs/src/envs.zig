const std = @import("std");
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

fn requestFn(resp: *worker.Response, r: *worker.Request) void {
    _ = r;

    const envvar = std.process.getEnvVarOwned(allocator, "MESSAGE") catch "";
    defer allocator.free(envvar);

    const s =
        \\The environment variable value is: {s}
    ;

    var body = std.fmt.allocPrint(allocator, s, .{ envvar }) catch undefined; // add useragent

    _ = &resp.headers.append("x-generated-by", "wasm-workers-server");
    _ = &resp.writeAll(body);
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
