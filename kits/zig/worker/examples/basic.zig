const std = @import("std");
const io = std.io;
const http = std.http;
const worker = @import("worker");

var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
const allocator = arena.allocator();

// Not working with *http.Server.Response
// fn cool(resp: *http.Server.Response, r: *http.Client.Request) void {
fn requestFn(resp: *worker.Response, r: *http.Client.Request) void {
    _ = r;
    std.debug.print("Hello from function\n", .{ });

    _ = &resp.httpHeader.append("content-type", "text/plain");
    _ = &resp.httpHeader.append("x-generated-by", "wasm-workers-server");

    _ = &resp.writeAll("Zig World!");
}

pub fn main() !void {
    worker.ServeFunc(requestFn);
}
