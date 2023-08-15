const std = @import("std");
const io = std.io;
const http = std.http;
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

// Not working with *http.Server.Response
// fn cool(resp: *http.Server.Response, r: *http.Client.Request) void {
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
